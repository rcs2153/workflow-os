# Proportional-Governance Runtime-Fact Freshness Plan

## 1. Executive Summary

The explicit proportional-governance path can reassess retries and approval
resume against exact typed runtime facts, but those facts were still supplied
directly by the caller without a source identity or freshness boundary. This
phase adds the smallest Core-owned model and same-call helper needed to obtain,
validate, bind, and assess current facts from one explicitly registered source.

This phase does not activate proportional governance in an executor. It does
not add automatic checks, persistence, schemas, CLI behavior, provider calls,
OpenShell integration, SideEffect execution, or writes.

## 2. Goals

- Register one injected runtime-fact source by stable identity and contract
  version.
- Bind the registration to a credential-free configuration commitment.
- Request facts for one exact stored immutable run bundle.
- Validate source identity, contract version, bundle binding, observation time,
  and exact workflow-step coverage.
- Apply the stricter of source-owned and Core-owned freshness limits.
- Derive the existing immutable-bundle governance assessment in the same call.
- Return a payload-free accepted snapshot with the assessment.
- Use stable non-leaking errors and redaction-safe Debug behavior.

## 3. Non-Goals

- Executor integration or default activation.
- Persistence or replay authorization.
- Cryptographic source authentication or signed attestation.
- Automatic local check execution.
- Multi-step execution expansion.
- Runtime or workflow schema fields.
- CLI or UI exposure.
- Provider calls, provider writes, OpenShell execution, or SideEffect execution.
- Hosted source registration or enterprise identity.

## 4. Trust Boundary

The embedding caller explicitly selects both a validated source registration
and an injected source implementation. This is a local trust decision. Matching
source IDs and contract versions prevent accidental source substitution inside
the helper; they do not prove remote identity, code provenance, or operator
authority.

Serialized source registrations and accepted snapshots are records and
commitments, not reusable authority. Accepted snapshots intentionally have no
deserialization path.

## 5. Candidate Model

- `GovernanceRuntimeFactSourceId`
- `GovernanceRuntimeFactSourceContractVersion`
- `GovernanceRuntimeFactSnapshotId`
- `GovernanceRuntimeFactSourceRegistration`
- `GovernanceRuntimeFactSourceRequest`
- `GovernanceRuntimeFactObservation`
- `GovernanceRuntimeFactSource`
- `GovernanceRuntimeFactSnapshot`
- `GovernanceRuntimeFactAssessment`

The source request contains the exact immutable bundle binding, the Core-owned
evaluation time, and read-only access to the already validated stored bundle.
The source observation contains only typed runtime facts and bounded identity,
time, and bundle-binding metadata.

## 6. Freshness And Coverage Rules

- Observation time must not be later than evaluation time.
- Source and Core freshness bounds must both be valid and nonzero.
- Effective maximum age is the smaller bound.
- An observation older than the effective bound fails closed.
- Source identity and contract version must equal the registration.
- Observation bundle binding must equal the stored immutable bundle.
- Exactly one runtime-fact record must exist for every immutable workflow step.
- Fact order must not change commitments or assessment order.

## 7. Privacy And Error Posture

Source failures are replaced by a stable Core-owned error. Public errors do not
include source output, IDs, hashes, paths, facts, credentials, or payloads.
Identifiers reject secret-like text. Debug output redacts source, snapshot,
bundle, time, and commitment values. No raw provider, command, parser, spec,
environment, or credential payload is modeled.

## 8. Test Plan

- Fresh registered observations produce a bound snapshot and assessment.
- The stricter Core age bound rejects stale observations.
- Future-dated observations fail closed.
- Source and bundle identity mismatches fail closed.
- Missing or duplicate step coverage fails closed.
- Fact order is canonical.
- Fact changes invalidate fact and snapshot commitments.
- Source failures and Debug output do not leak secret-like values.
- Serialized snapshots contain no forbidden raw payload fields.
- Invalid identifiers and freshness bounds fail closed.

## 9. Implementation Sequence

1. Add bounded source, contract-version, and snapshot identities.
2. Add explicit source registration and read-only source request models.
3. Add untrusted observation and injected source interface.
4. Add same-call freshness, identity, binding, and coverage validation.
5. Reuse the accepted immutable-bundle assessment helper.
6. Return a payload-free accepted snapshot and assessment together.
7. Add focused tests and phase documentation.
8. Complete maintainer review before any executor consumer is added.

## 10. Acceptance Criteria

- Caller-supplied detached runtime facts are no longer the only available
  boundary for future trusted consumption.
- Fresh facts are obtained once, validated against an explicit registration and
  exact immutable bundle, and assessed in the same call.
- Stale, future-dated, mismatched, incomplete, or source-failed observations
  fail closed without leakage.
- No runtime behavior, persistence, schema, CLI, provider, or write capability
  is added.

## 11. Recommended Next Phase

After focused maintainer review, plan one explicit opt-in executor consumer of
this same-call helper. That consumer must preserve existing workflow semantics,
must not make proportional governance a default, and must settle what durable
snapshot commitment is required for retry and approval-resume reassessment.
