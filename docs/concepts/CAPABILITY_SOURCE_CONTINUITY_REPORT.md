# Capability Source Continuity Report

## 1. Executive Summary

Workflow OS now has one Core-owned durable source-continuity boundary for the
accepted capability-gated GitHub repository-metadata read. The production
input no longer accepts caller-authored first/prior posture. Core prepares the
exact read, atomically reconciles its registered source and query against the
managed local SQLite state, and only then permits the injected read-only
adapter client to run.

This remains a private local vertical slice. It adds no executor/public API,
second provider operation, mutation, workflow configuration, SDK, CLI, hosted
identity, reusable authority, OpenShell integration, or release change.

## 2. Scope Completed

- Added a private durable continuity store to the managed SQLite backend.
- Added explicit V7-to-V8 schema migration and V8 manifest validation.
- Bound continuity to domain-separated source-registration and exact-query
  commitments without storing repository identity.
- Added atomic first observation, replay, monotonic advancement, rollback
  rejection, substitution rejection, and conflict handling.
- Composed continuity after existing operation preparation and before adapter
  invocation.
- Removed caller-authored continuity from the production operation input.
- Preserved the existing read outcome and authority receipt behavior.

## 3. Scope Explicitly Not Completed

No second GitHub operation, provider mutation, automatic adapter gating,
executor integration, public capability API, workflow schema, SDK, CLI,
runtime configuration, inferred grant, hosted identity, enterprise RBAC,
nested harness execution, OpenShell integration, or release-posture change was
added.

## 4. Storage And Migration Summary

SQLite schema V8 adds `capability_source_continuity`. Each row contains only a
continuity-key commitment, source-registration commitment, exact-query
commitment, optional source generation, and opaque watermark. Fresh databases
create V8 directly. Existing V7 databases require the explicit
`upgrade_capability_source_continuity_v7_to_v8` operation; ordinary open does
not migrate implicitly.

The schema remains bound to its managed manifest digest. The new table is
empty when a filesystem migration creates a fresh SQLite destination because
the filesystem backend has no prior capability-source continuity family to
infer.

## 5. Atomicity And Failure Ordering

Compare-and-advance uses a SQLite `IMMEDIATE` transaction. Concurrent exact
observations converge to one creator and one replay. Conflicting observations
at the same generation produce one winner and one deterministic rejection.

Policy, repository identity, source freshness, exact-query coverage, capability
authority, projection, and adapter-request validation all complete before the
continuity transaction. The transaction commits before provider invocation.
Continuity or storage failure therefore produces zero provider calls. A
provider failure preserves the already-observed source generation, and a later
rollback attempt remains blocked.

## 6. Privacy And Error Posture

The durable row stores commitments and an opaque watermark only. It stores no
grant, availability record, GitHub payload, repository name, credential,
authorization header, environment value, command output, source path, or raw
authority fact.

Errors use stable bounded
`current_authority.github_repository_read.source_continuity.*` codes. They do
not echo continuity keys, source/query commitments, generations, watermarks,
repository input, paths, or provider data. Debug formatting of prior
continuity continues to redact watermarks.

## 7. Test Coverage

Focused coverage proves:

- first observation and restart reconstruction;
- exact replay;
- higher-generation monotonic advancement;
- generation rollback rejection;
- missing current generation rejection;
- same-generation watermark substitution rejection;
- invalid operation preparation does not advance continuity;
- provider failure preserves accepted continuity;
- one-winner concurrent creation/replay;
- one-winner conflicting concurrent observation;
- source and query binding substitution rejection;
- zero adapter calls on continuity failures;
- payload-free stored state and non-leaking errors;
- fresh V8 schema creation and complete V1-through-V8 explicit upgrade; and
- state-migration schema-version compatibility.

## 8. Governed Phase Record

- workflow ID: `dg/implement`
- run ID: `run-1792698511475689000-2`
- approval ID: `approval/run-1792698511475689000-2/implementation-approved`
- presentation ID: `presentation/526eacd6aeec3a31`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: persisted proof enforced
- kernel boundary: governance coordination only; implementation, tests,
  documentation, validation, Git, pull-request, CI, and merge work remained
  outside the kernel

## 9. Validation

- focused current-authority GitHub read tests: passed, 21 tests
- focused continuity-store tests: passed, 3 tests
- SQLite backend tests: passed, 17 tests
- state-migration writer-guard tests: passed, 11 tests
- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed on the final full rerun
- `npm run check:docs`: passed
- `git diff --check`: passed

The first full workspace test invocation observed one failure in the existing
`competing_process_owners_admit_at_most_one_executor_entry` concurrency test:
one losing process owner coherently observed an already-terminal run with zero
entries, a result the test assertion did not accept. The exact test passed on
immediate rerun, and the complete workspace passed on the subsequent full
rerun, including that test. No implementation or assertion was changed to hide
the observation.

## 10. Remaining Limitations

- The durable composition remains crate-private and local SQLite-only.
- Existing direct adapter APIs remain outside the capability-gated claim.
- Continuity state proves source monotonicity; it is not a grant, reusable
  authority, provider-result receipt, or hosted identity assertion.
- No executor or workflow-schema path selects this operation yet.
- The dependency-tracked draft-pull-request live sandbox smoke remains
  separately blocked on its dedicated credential.

## 11. Recommended Next Phase

Perform the focused maintainer/security review of this durable continuity
boundary. Verify migration safety, exact binding, race behavior, failure
ordering, replay after provider failure, privacy, and the absence of public or
write-surface broadening before selecting any executor-facing adoption or
second provider operation.
