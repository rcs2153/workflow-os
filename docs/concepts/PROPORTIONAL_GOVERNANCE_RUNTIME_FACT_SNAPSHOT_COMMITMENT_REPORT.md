# Proportional-Governance Runtime-Fact Snapshot Commitment Report

## 1. Executive Summary

Workflow OS now durably commits the accepted current-runtime-fact observation
that establishes an opt-in executor assessment. The new payload-free binding is
nested atomically inside assessment-binding V3 and remains provenance metadata,
not reusable authority.

## 2. Scope Completed

- Added a versioned, self-validating runtime-fact snapshot commitment binding.
- Bound source registration, immutable bundle, initial snapshot, canonical fact
  set, freshness inputs, fact count, and assessment aggregate.
- Added assessment-binding V3 while preserving V1 and V2.
- Persisted and event-projected the commitment through the existing atomic
  assessment-binding path.
- Re-resolved facts on exact retry and retained the initial commitment.
- Rejected changed registrations, changed assessments, and corrupt durable
  records before new events or duplicate execution.
- Added focused serde, privacy, persistence, corruption, and retry tests.

## 3. Scope Explicitly Not Completed

The phase did not add approval-resume source consumption, reusable authority,
raw fact persistence, default activation, disposition enforcement, automatic
checks, provider execution, OpenShell, SideEffects, writes, schemas, CLI
behavior, hosted behavior, or mutation expansion.

## 4. Model And API Summary

`GovernanceRuntimeFactSnapshotBindingVersion::V1` identifies the payload-free
commitment model. `GovernanceRuntimeFactSnapshot::commitment_binding` creates a
validated binding from an accepted same-call snapshot.

`GovernanceAssessmentBindingVersion::V3` contains exactly one runtime-fact
snapshot binding. Its public read-only accessor supports inspection without
exposing raw runtime facts.

## 5. Validation Boundary

Deserialization rejects unknown fields and versions, empty or oversized fact
counts, invalid age bounds, impossible freshness relationships, and commitment
mismatch. The outer V3 binding also requires exact immutable-bundle and
assessment-aggregate agreement.

## 6. Retry And Durability Semantics

Fresh execution persists V3 before run events. Exact retry resolves a new
source observation and requires the same source-registration commitment,
immutable bundle, and resulting governance assessment. Snapshot identity and
observation time may change. The initial commitment remains durable and is not
silently replaced.

## 7. Privacy And Error Summary

The durable binding contains bounded metadata and hashes only. It stores no raw
runtime facts, source payloads, provider output, command output, credentials, or
tokens. Debug output redacts identifiers, bundles, commitments, and timestamps.
Malformed serialized values produce fixed non-leaking errors.

## 8. Test Coverage Summary

Focused tests cover V1 representation, serde round trip, tamper and unknown
version rejection, payload absence, Debug safety, V3 persistence, initial
snapshot linkage, equivalent retry, changed facts, changed registration,
corrupt durable state, event stability, and no duplicate execution.

## 9. Commands Run And Results

- Focused runtime-fact source tests: passed.
- Focused source-backed local executor tests: passed.
- Focused workflow-core clippy: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

- Approval resume does not resolve current facts from the registered source.
- The source registration is a local trust decision, not remote attestation.
- The generic executor path records rather than enforces the selected
  proportional-governance disposition.
- Report citation of the durable snapshot commitment is not implemented.
- No project or workflow schema selects a source.

## 11. Recommended Next Phase

Implement a separately reviewed approval-resume source consumer. It must use
the stored initial commitment as provenance, resolve fresh decision-time facts,
and fail closed on source, bundle, or assessment mismatch.

## 12. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1786281020359413000-2`
- Approval ID: `approval/run-1786281020359413000-2/composition-approved`
- Presentation ID: `presentation/179d2d66df12050c`
- Approval outcome: granted with persisted presentation proof
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Approval-presentation enforcement: proof enforced with event marker present
- Out-of-kernel work: implementation, tests, documentation, validation, and
  git/PR operations
- Skipped checks: opt-in live integration tests remained intentionally ignored;
  no live provider or OpenShell execution was in scope
