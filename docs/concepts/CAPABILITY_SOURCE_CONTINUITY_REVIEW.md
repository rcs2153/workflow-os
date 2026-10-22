# Capability Source Continuity Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The implemented private local boundary satisfies the accepted security
invariant: Core, rather than an adopting caller, decides whether an exact
capability-source observation is first, replayed, or advanced. Durable state is
bound to the registered source and exact GitHub repository query, transition
validation is serialized in one immediate SQLite transaction, and every
continuity rejection occurs before the injected provider client can run.

No blocker remains for this crate-private, opt-in, read-only vertical slice.
This acceptance does not authorize executor-facing adoption, a public API, a
second provider operation, or any provider mutation.

## 2. Scope Verification

The phase stayed within its approved boundary. It added:

- one private SQLite-backed source-continuity store;
- one private composition method for the already accepted GitHub repository
  metadata read;
- one additive, explicit V7-to-V8 SQLite migration;
- focused transition, concurrency, privacy, recovery, and migration tests; and
- implementation documentation.

It did not add another provider operation, provider mutation, automatic adapter
gating, executor or public capability APIs, workflow configuration, schema or
SDK exposure, CLI behavior, automatic grants, inferred authority, hosted
identity, enterprise RBAC, OpenShell integration, nested harness execution, or
release changes.

## 3. Durable Binding Assessment

The continuity key is a domain-separated commitment over the registered source
commitment and the exact repository-query commitment. The row also stores both
commitments and verifies them on every read. This redundant binding is useful:
the lookup key selects the record while the stored commitments fail closed if
state is substituted or corrupted.

The production-compiled input no longer carries caller-authored
`CapabilityUseSourceContinuity`. The durable method accepts the current
registered source, managed SQLite state, and the existing operation input. The
caller therefore cannot reset established history by selecting
`FirstObservation`.

Continuity does not authorize the operation. Policy, canonical repository
identity, source freshness and completeness, exact-query coverage, capability
resolution, step-scoped projection, and adapter-request validation remain
separate prerequisites.

## 4. Transition And Atomicity Assessment

`compare_and_advance_capability_source_continuity` uses an SQLite `IMMEDIATE`
transaction. The transaction reads, validates, conditionally updates or
creates, and commits one continuity row before returning. Competing writers are
serialized at the database boundary.

The transition matrix correctly:

- accepts an exact replay;
- accepts a higher versioned generation and its new watermark;
- permits the one-way transition from unversioned to versioned history;
- rejects versioned-to-unversioned regression;
- rejects generation rollback;
- rejects same-generation watermark substitution; and
- rejects changing an unversioned watermark.

A commit error is reported as an ambiguous outcome. The provider is not called
after that ambiguity. Retrying the exact observation safely reconciles as
either first observation or replay according to the committed database state.

## 5. Provider Ordering And Failure Assessment

The operation follows this order:

1. validate and prepare the existing bounded read operation;
2. compare and advance source continuity atomically;
3. recheck the reconstructed prior posture against the prepared source;
4. invoke the injected read-only provider client; and
5. construct the payload-free authority receipt after successful read output.

Every preparation or continuity failure returns `BlockedBeforeRead` and causes
zero provider calls. Provider failure remains distinct as `ReadFailed`.

The accepted source observation remains committed when the later provider call
fails. This is correct for this model: continuity records the authoritative
source generation observed during admission, not provider-operation success.
A retry of the same source reconciles as an exact replay and can attempt the
read again.

The provisional `First` posture used while preparing the current source
snapshot is not serialized into that snapshot or receipt. The snapshot commits
the current registration, request, generation, watermark, records, and
freshness. After SQLite returns the real prior posture, the method validates it
before provider invocation. Replay therefore does not emit a false durable
claim that the source was first observed.

## 6. Migration And Compatibility Assessment

Fresh databases create schema V8. Existing exact V7 databases require the
explicit `upgrade_capability_source_continuity_v7_to_v8` operation; ordinary
open does not migrate implicitly. The upgrade:

- validates the complete V7 manifest and existing continuity security state;
- adds only the V8 table;
- updates managed metadata and `user_version` in the same immediate
  transaction;
- validates the complete V8 manifest before commit; and
- is idempotent only for an already-valid V8 database.

The complete V1-through-V8 upgrade test preserves existing event state and
proves that the new continuity table begins empty. The state-migration writer
guard is aligned to schema version 8. No existing public serialization shape or
workflow schema changed.

## 7. Privacy And Error Assessment

Durable state contains:

- a continuity-key commitment;
- a source-registration commitment;
- an exact-query commitment;
- an optional numeric generation; and
- the source watermark, which this registered source derives from its
  inventory commitment.

It contains no repository owner/name, grant, availability record, GitHub
payload, credential, authorization header, environment value, command output,
source path, or raw authority fact.

Continuity errors use stable
`current_authority.github_repository_read.source_continuity.*` codes and fixed,
bounded messages. They do not echo identifiers, commitments, generations,
watermarks, repository values, paths, or provider data. Debug formatting
redacts prior watermarks and operation outcomes.

## 8. Test Quality Assessment

Focused coverage proves:

- first observation and restart reconstruction;
- exact replay and monotonic generation advancement;
- rollback, missing-generation, same-generation substitution, and unversioned
  substitution rejection;
- binding substitution rejection;
- zero provider calls on invalid preparation and continuity failure;
- provider failure followed by safe replay;
- one-winner concurrent exact creation/replay;
- one-winner same-generation conflicting observations;
- fresh V8 creation, explicit V7-to-V8 upgrade, idempotent exact-V8 handling,
  and full prior-version upgrade sequencing; and
- stable non-leaking errors and payload-free stored commitments.

The tests exercise real SQLite transactions and reopen the database to prove
restart behavior. Existing workspace tests also preserve the surrounding
EvidenceReference, Diagnostic, adapter, runtime, report, and state behavior.

## 9. Blockers

None for the accepted private, local, read-only boundary.

## 10. Non-Blocking Follow-Ups

1. Add a process-level contention test when this private helper becomes an
   executor-facing or multi-worker adoption path. The current thread-level test
   exercises independent SQLite connections and the database lock, but a
   process boundary would better represent future worker deployment.
2. If privacy regression tooling is generalized, inspect the SQLite main file,
   WAL, and shared-memory sidecars together. The current implementation stores
   only commitments, but the existing byte-scan assertion covers the main file
   only.
3. Keep continuity semantically separate from operation success and reusable
   authority. Future receipts may disclose a payload-free continuity posture,
   but must not turn a prior observation into authorization for another action.

These follow-ups do not justify broadening the present phase.

## 11. Documentation Review

The plan and implementation report accurately describe the boundary as private,
local, SQLite-backed, and read-only. They explicitly exclude provider
mutation, public/executor adoption, workflow configuration, hosted identity,
OpenShell, reusable authority, and release changes.

The documentation correctly states that provider failure preserves a valid
source observation and that continuity admission precedes provider invocation.
No dangerous capability overclaim was found.

## 12. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1792700523992380000-2`
- approval ID:
  `approval/run-1792700523992380000-2/review-scope-approved`
- presentation ID: `presentation/28a2dbde226425c3`
- presentation content hash:
  `28a2dbde226425c3fdecf6b676134e34c427f585ad8521a6cbf29b52b14aea03`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- kernel boundary: governance coordination only; source review, repository
  inspection, validation, documentation, Git, pull-request, CI, and merge work
  remain outside the kernel

## 13. Recommended Next Phase

**Reconcile the Active Delivery Queue before selecting another implementation
slice.**

The source-continuity limitation that selected this phase is now closed. The
dedicated live sandbox proof remains dependency-tracked. Queue reconciliation
should choose one executable runtime-composition boundary from the already
accepted foundations and should not authorize a second provider operation or
provider mutation merely because this private read boundary passed review.
