# Trusted Host TimeWindow Wait Binding Implementation Report

## 1. Executive Summary

Workflow OS now durably binds a `DeadlineReached` wait to one exact trusted
time dependency and provides a crate-private verifier that can satisfy that
wait only from a fresh Core-obtained trusted-time observation.

This is prerequisite hardening, not caller integration. No scheduler, polling
loop, executor recirculation, provider execution, sandbox integration, or
public wake API was added.

## 2. Scope Completed

- Added a private authoritative `TimeWindow` dependency binding containing the
  deadline, trusted-time source, provenance commitment, epoch identifier, and
  a domain-separated commitment over those values.
- Included dependency bindings in register-yield and wait-transition request
  commitments.
- Required `DeadlineReached` waits to carry a valid exact binding at
  registration.
- Added a crate-private SQLite verifier that obtains trusted time internally,
  verifies the durable binding, and constructs the private wake capability.
- Added SQLite schema v6 and an explicit v5-to-v6 upgrade.
- Preserved legacy wait rows as unbound and made automatic TimeWindow wake fail
  closed for those rows.
- Added focused success, early-wake, malformed-deadline, forged-commitment,
  legacy-row, and schema-upgrade coverage.

## 3. Scope Explicitly Not Completed

This phase did not add:

- a trusted-host caller or operational-entry caller integration;
- supervisor production of non-empty wait declarations;
- a scheduler, timer, poller, queue, daemon, or wake bus;
- model-turn creation or automatic executor recirculation;
- approval, evidence, check, provider, or capability wake verifiers;
- provider execution or mutation, OpenShell, or sandbox integration;
- nested harnesses, recursive agents, or agent swarms;
- public APIs, configuration, workflow schema, CLI, SDK, UI, or examples;
- filesystem, PostgreSQL, multi-host, hosted, or distributed parity; or
- release-posture changes.

## 4. Model And Commitment Boundary

`AuthoritativeWaitRecord` may carry a private
`AuthoritativeWaitDependencyBinding::TimeWindow`. The binding covers:

- the exact deadline;
- `CoreInjectedClockV1` as the trusted-time source;
- the trusted-time provenance commitment;
- the trusted-time epoch identifier; and
- a domain-separated dependency commitment derived from those values.

Registration rejects a missing binding, a mismatched trigger, a forged
commitment, a source/provenance/epoch mismatch, a deadline at or before the
registration observation, or a deadline beyond the authorized execution
window.

## 5. SQLite Schema And Migration

SQLite adapter schema v6 adds nullable columns for the TimeWindow dependency
shape. The explicit v5-to-v6 upgrade changes schema only; it does not infer or
backfill authority into existing waits. A legacy `DeadlineReached` row with no
binding therefore remains representable for migration but cannot be
automatically satisfied.

Current-schema validation requires dependency columns to be either all null or
a complete valid TimeWindow shape. Persistence and readback validate both the
relational projection and canonical JSON record.

## 6. Private Verifier

The crate-private verifier accepts only expected identities, revisions,
cursor, and immutable window binding. It then:

1. reloads authoritative continuity state;
2. verifies the exact window and wait revisions;
3. rejects legacy unbound deadline waits;
4. obtains a fresh trusted-time observation inside Core;
5. verifies source, provenance, epoch, deadline, watermark, and window expiry;
6. constructs the private wake capability carrying the exact dependency
   commitment; and
7. applies the ordinary atomic wait transition.

The caller cannot provide a boolean "deadline reached" assertion or construct
the private capability.

## 7. Validation And Error Boundary

Validation is deterministic and fail closed. Stable error codes distinguish
invalid dependency registration, an unsatisfied time window, an unbound legacy
wait, stale revisions, and wake-binding mismatch. Errors and Debug output do
not include raw dependency values or trusted-time provenance.

## 8. Test Coverage

Focused tests cover:

- exact binding persistence and successful transition at the deadline;
- rejection before the deadline;
- rejection of a deadline beyond the authorized window;
- rejection of a forged dependency commitment;
- rejection of legacy unbound deadline waits; and
- the explicit schema-upgrade chain through v6.

The full workspace suite and repository documentation checks are required for
phase close.

## 9. Commands Run And Results

- `cargo test -p workflow-core --lib sqlite_time_window -- --nocapture`:
  passed, 4 tests.
- `cargo test -p workflow-core --test sqlite_state_backend sqlite_backend_requires_each_explicit_schema_upgrade_through_v6 -- --nocapture`:
  passed, 1 test.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live provider and local-check tests
  remained ignored as designed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

- No accepted runtime path currently registers a genuine TimeWindow wait.
- No private trusted-host caller invokes the verifier.
- The verifier is local SQLite only.
- Other wake-source families still have trigger vocabulary without exact
  dependency verifiers.
- No scheduler or external execution substrate exists.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of this implementation. Do not
integrate a caller until the review confirms commitment coverage, migration
posture, trusted-time freshness, replay/concurrency behavior, and privacy.

## 12. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1791023045926049000-2`
- approval: `approval/run-1791023045926049000-2/implementation-approved`
- presentation: `presentation/7874c1870bf95f42`
- presentation hash:
  `7874c1870bf95f422ae30ffcc30ee51707d42116b9f2f0ec1f13214765ed14fc`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: exact durable TimeWindow wait binding, SQLite v6, private
  verifier, focused tests, and honest documentation only
- phase status: completed after successful validation and governed close
- event summary: 39 events, including 1 approval request, 1 approval grant,
  8 policy decisions, 6 scheduled steps, 6 successful skill invocations, 0
  retries, and 0 escalations; approval-presentation proof enforced
- validation summary: focused TimeWindow and schema migration tests, formatting,
  workspace Clippy, the complete workspace test suite, documentation checks,
  and diff hygiene passed; opt-in live tests remained skipped as designed
- out-of-kernel work: implementation, tests, validation, documentation, and
  git actions were performed by the delegated trusted host; Workflow OS
  governed scope and approval but did not perform those actions
