# Production Current-Authority Source Boundary Model Report

## 1. Executive Summary

Workflow OS now has a payload-free model for the production
current-authority source boundary.

The model can commit source identity, exact request scope, coherent snapshot
posture, completeness, consistency, freshness, and bounded failure state. It
cannot authenticate a source, read authority state, confer readiness,
authorize target access, or participate in runtime execution.

## 2. Scope Completed

- Added bounded source, contract, snapshot, watermark, and generation types.
- Added explicit fact-family, completeness, consistency, freshness, failure,
  and retry-posture vocabulary.
- Added a model-only source registration commitment.
- Added an exact source request derived from the immutable execution binding,
  required-context contract, and canonical query set.
- Added a payload-free coherent snapshot commitment with canonical family and
  count validation.
- Added stricter-of-source-and-Core freshness evaluation.
- Added stable non-leaking validation errors.
- Added serde and redaction-safe `Debug` behavior.
- Exported the model from `workflow-core`.
- Added focused unit and integration tests.

## 3. Scope Explicitly Not Completed

This phase does not add a trusted source registration boundary, source trait,
registry, concrete source, store, filesystem or network access, resolver
integration, runtime consumer, readiness API, target dereference, persistence,
events, audit projection, receipts, artifacts, providers, OpenShell, sandbox
execution, SideEffect execution, writes, schemas, SDKs, CLI behavior, UI,
examples, hosted behavior, reasoning lineage, or release changes.

## 4. Model Types Added

- `CurrentAuthoritySourceModelVersion`
- `CurrentAuthoritySourceId`
- `CurrentAuthoritySourceContractVersion`
- `CurrentAuthoritySourceSnapshotId`
- `CurrentAuthoritySourceWatermark`
- `CurrentAuthoritySourceGeneration`
- `CurrentAuthoritySourceKind`
- `CurrentAuthorityFactFamily`
- `CurrentAuthoritySourceConsistency`
- `CurrentAuthoritySourceCompleteness`
- `CurrentAuthoritySourceFreshness`
- `CurrentAuthoritySourceRegistration`
- `CurrentAuthoritySourceRequest`
- `CurrentAuthoritySourceReadWindow`
- `CurrentAuthoritySourceFactCount`
- `CurrentAuthoritySourceSnapshot`
- `CurrentAuthoritySourceFailureKind`
- `CurrentAuthoritySourceFailurePosture`
- `CurrentAuthoritySourceFailure`

## 5. Trust Boundary

`CurrentAuthoritySourceRegistration` is a model-only commitment. Its public
constructor validates bounded identity, configuration commitment, fact-family,
consistency, freshness-cap, sensitivity, and redaction posture, but it does
not authenticate a source.

A future Core-owned runtime boundary must decide which registrations and source
implementations are trusted. Serialized registrations and caller-built
snapshots remain data and cannot establish authority or readiness.

## 6. Request Boundary

`CurrentAuthoritySourceRequest` binds:

- the accepted registration commitment;
- immutable execution-binding hash;
- required-context contract hash;
- canonical exact query-set hash and count;
- requested fact families;
- execution sensitivity bounded by source registration; and
- caller-supplied evaluation time.

The constructor derives the query set from the validated contract and rejects
contract substitution, unsupported families, pre-binding evaluation time, and
sensitivity above the registered source ceiling.

## 7. Snapshot, Watermark, And Generation

`CurrentAuthoritySourceSnapshot` commits one aggregate read. It retains opaque
snapshot and watermark identities, an optional non-zero source-defined
generation, a validated read window, exact requested and returned family
coverage, bounded fact counts, records commitment, and snapshot commitment.

Watermarks deliberately support equality only. They do not implement ordering.
Only `CurrentAuthoritySourceGeneration` may represent source-defined ordering,
and the model does not decide which future source contracts may safely use it.

## 8. Completeness, Consistency, And Freshness

`CompleteForExactQuery` requires exact requested-family coverage. Availability
and governed-context-reference counts must equal the exact query count. An
empty grant result remains valid when the complete source view contains no
matching grants.

Snapshot consistency must equal the registered source consistency. Atomic and
stable-watermark posture are representable; unknown consistency cannot be
accepted through registration.

Freshness uses explicit injected times. The effective validity bound is the
earlier of the source-supplied bound and the Core-owned maximum observation
age. Future-dated and stale snapshots remain explicit vocabulary and cannot
silently become fresh.

## 9. Failure And Retry Posture

The payload-free failure model distinguishes unavailable, unsupported,
incomplete, stale, future-dated, concurrent-change, ambiguous, corrupt,
registration-mismatch, query-mismatch, transport, and internal failures.

It records only registration and request commitments plus bounded failure and
future retry posture. This phase does not perform retries.

## 10. Privacy And Redaction

The model stores identifiers, bounded enums, timestamps, counts, and
commitments. It does not store credentials, provider or sandbox payloads,
source contents, target contents, command output, environment values, raw
configuration, paths, endpoints, database cursors, or unbounded errors.

Registration requires redaction posture. Debug output redacts identities,
commitments, timestamps, snapshot IDs, and watermarks. Custom deserialization
fails closed with bounded messages that do not echo rejected values.

## 11. Test Coverage

Focused tests cover:

- valid registration, request, and snapshot round trips;
- source identifier and generation bounds;
- canonical family ordering and deterministic registration commitments;
- duplicate and unsupported family rejection;
- contract and registration substitution;
- source sensitivity ceilings and mandatory redaction;
- exact family and target coverage;
- empty-but-complete grant results;
- all bounded completeness postures;
- atomic and stable-watermark consistency;
- read-window ordering;
- source and Core freshness bounds;
- stale and future-dated posture;
- wire tampering and invalid enum failure;
- all source failure categories;
- payload-free serialization and redaction-safe Debug; and
- absence of readiness, authorization, source-service, or runtime APIs.

## 12. Validation Commands And Results

- `cargo test -p workflow-core --test current_authority_source`: passed,
  10 tests.
- focused source module unit tests: passed, 5 tests.
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Remaining Known Limitations

- No Core-owned trusted registration or source instantiation exists.
- No concrete source can obtain current facts.
- Registration freshness is an explicit numeric cap rather than a runtime
  policy reference.
- Prerequisite decision fact families remain future vocabulary.
- No source-backed same-call assessment or one-time-use boundary exists.
- The model cannot confer readiness, dereference targets, or lower
  proportional-governance friction.
- OpenShell remains a separate future execution-provider concern.

## 14. Recommended Next Phase

Perform a focused maintainer review of this model.

If accepted, proceed to the private registered-source interface proof with one
in-memory aggregate source. Do not add runtime consumers, dereference,
providers, OpenShell integration, SideEffect execution, or writes.

## 15. Governed Phase Record

- workflow: `dg/implement`
- run ID: `run-1785158190120755000-2`
- approval ID:
  `approval/run-1785158190120755000-2/implementation-approved`
- approval presentation ID: `presentation/9066540a87712be6`
- approval presentation content hash:
  `9066540a87712be6cc62f4f838336ffa7797b66af97171f4a0bed4b5734a3e04`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted implementation handoff was presented
- phase status: completed
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval-presentation proof enforced with a matching event marker
- out-of-kernel work: the delegated maintainer inspected architecture,
  implemented and tested the model, updated documentation, and ran validation;
  the kernel governed scope and approval but did not inspect code, edit files,
  execute checks, or mutate git
