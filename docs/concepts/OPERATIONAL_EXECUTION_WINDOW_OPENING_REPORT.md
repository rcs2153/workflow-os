# Operational Execution Window Opening Report

## 1. Executive Summary

Workflow OS now has a separately versioned, private operational execution-
window opening capability for explicit local SQLite state. Core rehydrates the
durable run, obtains registered current authority in the same call, binds the
exact `invoke_current_step_skill` operation, and commits one `executing`
window, one first `started` attempt, one bounded runtime event, and one derived
snapshot atomically.

This phase does not invoke the skill. It does not add the trusted-host
supervisor, scheduling, automatic approval, model-turn creation, provider
mutation, OpenShell execution, nested harness execution, CLI behavior, public
schema, or hosted runtime.

## 2. Scope Completed

- Added separately versioned opening operation and receipt identities.
- Added the closed `invoke_current_step_skill` operation binding.
- Added a private Core-only opening authorization and attempt-use capability.
- Added a same-call bridge from registered current authority to the SQLite
  opening transaction.
- Rehydrated run identity, non-terminal posture, immutable bundle, and exact
  runtime cursor before opening.
- Added atomic SQLite persistence for the opening operation, authoritative
  continuity window, separately identified first attempt, runtime event,
  snapshot, and projection binding.
- Added exact replay and same-identity/different-content conflict handling.
- Added fresh-connection reconciliation of the complete committed result.
- Added explicit schema V4 and explicit V3-to-V4 migration.
- Preserved the accepted closed five-operation continuity contract.

## 3. Scope Explicitly Not Completed

This phase does not add trusted-host invocation, skill dispatch, attempt
outcome consumption, yield registration through the new attempt capability,
resume scheduling, polling, a daemon, automatic approval, delegated self-
approval, arbitrary callbacks, provider writes, new mutation families,
OpenShell execution, filesystem or PostgreSQL opening support, workflow
schema, runtime config, CLI, SDK, examples, nested harnesses, Reasoning
Lineage, enterprise administration, or release changes.

## 4. Opening API And Authority Boundary

The opening store is a private V1 capability, distinct from the accepted
five-operation continuity store. Callers provide bounded identity, expiry,
attempt-budget, and trusted-time inputs, but cannot provide a serialized gate
assessment or reusable authority token.

Core rehydrates the durable run and enters the registered current-authority
source's same-call consumer. Only inside that consumer does Core derive the
source-authority commitment, Core governance commitment, and private opening
authorization. A blocked or failed authority source cannot call the store.

## 5. Exact Operation Binding

The opening is bound to exactly `invoke_current_step_skill` plus workflow,
run, and step identity. The commitment is persisted with the window-opening
operation and first attempt. A substituted step or operation commitment fails
closed before state mutation.

No generic callback, command, tool, provider operation, or broader action
vocabulary is authorized by this phase.

## 6. Atomic SQLite Boundary

The SQLite implementation uses one immediate transaction to:

1. detect exact replay or idempotency conflict;
2. rehydrate and validate the current run and cursor;
3. reject conflicting active window ownership;
4. validate and advance trusted time;
5. create the authoritative window in `executing` posture;
6. create attempt one in `started` posture;
7. append one bounded opening event;
8. derive and persist the run snapshot;
9. persist operation, attempt, and projection bindings; and
10. commit once.

Before-commit failure writes nothing. Exact replay returns the original
projection and creates no additional event or capability.

## 7. Event And Snapshot Projection

`OperationalExecutionWindowOpened` is a status-preserving runtime event
accepted only from running or retrying state. Its public payload is limited to
bounded identities, attempt/window revisions, the exact closed operation kind,
cursors, and commitments. The snapshot stores only the latest non-
authoritative opening projection.

Continuity state remains authoritative. The event and snapshot do not grant
execution authority and cannot invoke work.

## 8. Schema And Migration

SQLite schema V4 adds independent opening-operation, opening-attempt, and
opening-projection tables. It does not widen the closed continuity operation-
kind check.

The V3-to-V4 upgrade is explicit. It succeeds only when existing continuity
window and attempt tables are empty. A database containing pre-opening
continuity state fails with
`state.sqlite.schema.upgrade_opening_state_required`; the migration does not
invent opening authority for legacy or fixture-shaped state.

The original opening-only V4 schema manifest contained 35 objects and was
bound to digest
`35227c65da88e0a24b3d685e8160ecbbe5e7c7bafeb599968e757804f38ff87f`.
The follow-on trusted-host supervisor slice retained 35 objects while adding a
closed opening-origin discriminator to the empty continuity-attempt table. The
current post-migration V4 digest is
`1016e62bd4d5b3e27f29822212cfe771de7287206799a9135d8f054abb5c5aeb`.

## 9. Replay And Reconciliation

An exact operation replay returns the original durable result and projection
without another event. Reusing the operation identity with a different
receipt or request commitment fails closed. A fresh SQLite connection can
reconcile a known operation, request commitment, and receipt to either the
complete committed result or confirmed absence.

The implementation returns no private attempt-use capability from replay or
reconciliation. A future host must not infer dispatch authority from durable
records alone.

## 10. Privacy And Redaction

Opening identifiers are bounded, character-restricted, and reject secret-
looking values before storage. Debug output redacts identity and commitment
material. Serde errors do not echo invalid identifiers. Persisted and projected
records contain no prompts, transcripts, source/spec contents, command output,
provider payloads, environment values, paths, credentials, authorization
headers, private keys, approval reasons, evidence bodies, or reconstructable
capabilities.

## 11. Test Coverage

Focused tests cover:

- atomic opening with one window, first attempt, event, and snapshot;
- exact replay without event duplication;
- same-operation conflicting receipt rejection;
- stale runtime cursor rejection with zero writes;
- exact operation-binding substitution rejection with zero writes;
- fresh-connection reconciliation;
- secret-like identifier rejection and non-leaking errors;
- redaction-safe Debug and payload-free serialization;
- explicit V1-to-V2-to-V3-to-V4 migration;
- refusal to assign authority to pre-opening V3 continuity state; and
- existing SQLite conformance and Core regression behavior.

## 12. Validation

Completed validation:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- operational opening unit tests: passed, 9 tests across model and store;
- `cargo test -p workflow-core --test sqlite_state_backend`: passed, 17
  tests;
- `cargo test --workspace -j 2`: compiled the complete workspace and passed
  the CLI unit suite, CLI contract suite, read-only example suites, Core unit
  suite, and the adapter integration suite before the run was stopped because
  this macOS host imposed multi-minute launch latency on each separately
  linked integration-test binary. No test failed, but the complete workspace
  run is not claimed as passed;
- `npm run check:docs`: passed;
- `npm run check:integrations`: passed, including Core adapter contracts and
  CLI read-only example validation/run/approval/inspection; and
- `git diff --check`: passed.

An earlier isolated-target broad Core probe reached the local-executor suite
without a discoverable `workflow-os` binary or `npm`. The final validation
used the normal repository CLI build and bundled Node path, resolving that
environmental setup issue. The remaining incomplete all-binary workspace run
is disclosed as host launch latency rather than represented as a passing
result.

## 13. Remaining Known Limitations

- The private attempt-use capability is not yet consumed by any production
  host.
- The new first attempt cannot yet report success, failure, ambiguity, or
  yield through the accepted continuity operations without a separately
  reviewed bridge.
- No trusted-host supervisor or scheduler exists.
- No executor is invoked and no provider action occurs.
- Commit-acknowledgement fault injection and concurrent-opening contention
  need focused maintainer/security review to determine whether they are
  blocker-level additions before host dispatch.
- Filesystem and PostgreSQL opening support remain unavailable.
- SQLite remains local, opt-in, and not production-certified.

Fix-forward note: the subsequent one-shot trusted-host supervisor phase adds a
bounded opening-attempt origin bridge, so the first attempt can report a
projected yield or outcome without fabricating a consumed directive. It also
adds a private one-shot supervisor. Repeated scheduling and public runtime
integration remain unimplemented.

## 14. Recommended Next Phase

Perform a focused maintainer/security review of the operational opening
implementation. The review should decide whether concurrent-opener and commit-
fault proof must be added before accepting the opening capability. Only after
acceptance should Workflow OS implement the one-shot injected trusted-host
supervisor over this private capability.

## 15. Governed Phase Record

- workflow: `dg/implement`;
- run: `run-1790834464225217000-2`;
- approval: `approval/run-1790834464225217000-2/implementation-approved`;
- presentation: `presentation/03bceb35c798bb52`;
- presentation hash:
  `03bceb35c798bb52349f6b31194132daad72357425dd5cf5348ad7adbfe50641`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented; and
- phase close: `Completed`, 39 events, 1 approval, 0 retries, and 0
  escalations, with the approval-presentation proof marker present; and
- out-of-kernel work: the external executor edited source and documentation
  and ran validation. The complete all-binary workspace test run was stopped
  after passing major suites because this host imposed multi-minute launch
  latency per integration binary. The kernel did not edit files, run checks,
  commit source, invoke a skill, schedule work, or mutate a provider.
