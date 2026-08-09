# Proportional-Governance Runtime-Fact Snapshot Commitment Plan

## 1. Executive Summary

The opt-in source-backed executor can resolve current proportional-governance
facts and durably bind the resulting assessment, but it does not yet preserve
which accepted source observation established that assessment. This phase adds
one payload-free, self-validating source-snapshot commitment inside the existing
durable assessment binding.

The commitment proves initial provenance and integrity only. It does not make
old facts fresh, grant reusable authority, enforce a governance disposition, or
authorize approval-resume consumption.

## 2. Goals

- Commit the trusted source registration, exact immutable run bundle, initial
  accepted snapshot, canonical runtime-fact set, assessment aggregate,
  freshness inputs, and bounded fact count.
- Persist the commitment atomically with the existing assessment binding.
- Preserve V1 and V2 assessment-binding compatibility.
- Re-resolve current facts on exact retry.
- Accept a retry only when the same trusted source registration and immutable
  bundle produce the same governance assessment.
- Preserve the initial provenance commitment rather than replacing it during a
  retry.
- Keep Debug, serialization errors, and executor errors non-leaking.

## 3. Non-Goals

- Approval-resume source consumption.
- Reusable or delegated authority.
- Raw runtime-fact or source-payload persistence.
- Default proportional-governance activation or disposition enforcement.
- Automatic checks, provider execution, OpenShell integration, SideEffects,
  writes, schemas, CLI behavior, hosted behavior, or mutation expansion.

## 4. Core Model

Add `GovernanceRuntimeFactSnapshotBindingVersion` and
`GovernanceRuntimeFactSnapshotBinding`. The V1 binding contains only bounded
metadata and cryptographic commitments. It is self-validating on
deserialization and rejects unknown fields, unknown versions, invalid bounds,
invalid freshness relationships, and commitment mismatch.

The accepted call-local snapshot remains serialize-only. It can construct the
durable binding, but the resulting record explicitly remains provenance rather
than later execution authority.

## 5. Durable Binding Integration

Add `GovernanceAssessmentBindingVersion::V3` with exactly one nested runtime
fact snapshot binding. V1 remains the plain assessment binding and V2 remains
the authoritative local-check source binding. A record cannot combine the V2
and V3 source forms.

The nested binding must match the outer immutable bundle and assessment
aggregate. Existing create-only assessment persistence and event projection
then retain the full record atomically without a second store or event stream.

## 6. Fresh Execution And Retry Semantics

Fresh source-backed execution constructs V3 from the same-call accepted
snapshot and assessment set, persists it before run events, and returns the
call-local snapshot.

Exact retry must resolve current facts again. A changed snapshot ID, timestamp,
or fact-set commitment may be accepted only when the same registered source and
exact immutable bundle produce the same assessment aggregate and bounded
posture. Changed registration or changed assessment fails closed before new
events or duplicate execution. The initial durable snapshot commitment is not
rewritten.

## 7. Privacy And Error Posture

- Persist no raw fact vectors, source payloads, command output, provider output,
  credentials, tokens, or source-local error text.
- Redact commitments, identifiers, bundles, and timestamps from Debug output.
- Validate serialized records with stable Core-owned errors that do not echo
  malformed values.
- Keep source registration an explicit local embedding trust decision.

## 8. Test Plan

- Valid V1 snapshot commitment construction and serde round trip.
- Unknown version and tampered commitment fail closed without leakage.
- Serialized bindings contain no forbidden raw payload fields.
- Fresh executor runs persist V3 and bind the returned initial snapshot.
- Store round trip preserves the nested commitment.
- Exact retry accepts a new snapshot that yields the same assessment without
  re-execution or rebinding.
- Changed facts or changed source registration fail before new events.
- Corrupt persisted commitment fails before source observation or re-execution.
- Existing V1/V2 bindings and workspace behavior remain compatible.

## 9. Validation

- Focused runtime-fact source tests.
- Focused source-backed local executor tests.
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

## 10. Recommended Follow-Up

Plan and implement a separately reviewed approval-resume source consumer that
uses the durable initial commitment for provenance while resolving fresh facts
for the decision-time operation. Do not broaden provider mutations or defaults
first.
