# Authoritative Local-Check Executor Consumer Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation correctly composes canonical `DocsCheck` execution,
same-call authoritative reassessment, aggregate quiet-success enforcement, and
durable source commitment for an ordinary fresh invocation. It also preserves
existing executor defaults and the approved privacy boundary.

One blocker remains: fresh-run ownership is checked before immutable-bundle
publication but is not claimed atomically. Concurrent callers using the same
run ID can both pass the initial emptiness check, and the second caller may
accept the first caller's identical persisted bundle and execute the local
check again. That violates the accepted requirement that any existing bundle
fail before process use.

Fix-forward status: the create-only claim correction is implemented in
[Authoritative Local-Check Executor Consumer Blocker Fix Report](AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_BLOCKER_FIX_REPORT.md)
and accepted in the
[Authoritative Local-Check Executor Consumer Blocker Fix Review](AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_BLOCKER_FIX_REVIEW.md).
This note does not erase the original finding.

## 2. Scope Verification

The implementation stayed within the approved opt-in executor-consumer scope.

It did not add:

- default, automatic, background, parallel, or repository-wide checks;
- retry, rehydration, approval resume, or cancellation support;
- visible-disclosure continuation or proportional approval creation;
- additional check families;
- reports, artifacts, evidence attachment, CLI, UI, SDK, or schemas;
- providers, OpenShell, SideEffects, network access, or writes;
- hosted or distributed behavior;
- reasoning lineage;
- enterprise administration; or
- release changes.

## 3. API And Input Assessment

`LocalExecutionWithAuthoritativeDocsCheckGovernanceRequest` is narrow and
explicit. It reuses the immutable-run request and accepts only:

- one selected step;
- one governance profile;
- exact per-step runtime facts; and
- an optional expected aggregate fingerprint.

It does not accept local-check requirements, caller-selected check posture,
check result IDs, attestation IDs, detached facts, or a source binding. Core
derives those values from the canonical immutable declaration and a
length-framed identity algorithm.

The result returns the run, immutable binding, source-bound governance binding,
and bounded local-check result. Its `Debug` implementation exposes status,
counts, and bounded posture only.

## 4. Preflight And Ordering Assessment

The ordinary single-caller path has the intended ordering:

1. require an explicit run ID;
2. reject existing events or run-bound store material;
3. prepare the execution plan and evaluate pre-run policy;
4. build and validate the immutable bundle in memory;
5. resolve the selected canonical declaration and handler commitment;
6. derive Core-owned identities;
7. preflight the complete reassessment context;
8. publish and reload the immutable bundle;
9. execute the check and consume the private fact-bound reassessment;
10. enforce aggregate complete quiet `Proceed`;
11. persist the source-bound governance binding; and
12. append the existing run-start events and execute the workflow.

Tests prove that invalid immutable or caller-posture context wins over a later
process failure and that failed checks create no workflow events.

## 5. Aggregate Governance Assessment

The implementation enforces the complete multi-step binding, not only the
selected checked step.

Only this aggregate posture executes:

```text
execution=proceed
disclosure=quiet
completeness=complete
```

Visible disclosure, approval-required, denied, and incomplete results fail
before `RunCreated`. A passing selected check cannot weaken stricter facts on
another workflow step or another governance axis.

This closes the planning blocker identified before implementation.

## 6. Source-Binding Assessment

`GovernanceAssessmentBindingVersion::V2` adds one optional
`GovernanceAssessmentSourceBinding` containing:

- a bounded source kind;
- a versioned algorithm;
- a source fingerprint; and
- the selected step identity.

The constructor that claims authoritative local-check provenance remains
crate-private and consumes the private same-call bound assessment. Public
deserialization can reconstruct validated integrity data, but the executor
does not treat that data alone as current runtime authority.

V1 bindings remain readable without a source field. V1 with a source or V2
without a source fails closed. Binding equality and create-only persistence
retain the exact source commitment.

## 7. Event, Audit, Privacy, And Error Assessment

The existing governance-binding event retains the exact source commitment.
Its idempotency identity includes the aggregate assessment and source
commitment.

Audit projection exposes only aggregate posture, step count, and bounded source
kind. It does not expose selected-step identity, fingerprints, paths, commands,
process output, source contents, environment values, provider payloads,
credentials, or tokens.

New errors use stable `executor.authoritative_local_check.*` codes with static,
non-leaking messages.

## 8. Blocker

### P0: Fresh-run check ownership is not atomic

The initial guard at
`crates/workflow-core/src/executor.rs:7721` checks for existing event or bundle
state. After pure preflight, the new path calls
`persist_or_validate_immutable_run_bundle(...)` at
`crates/workflow-core/src/executor.rs:7788`.

That shared helper treats
`immutable_run_bundle_store.manifest_exists` as success when the stored
manifest equals the proposed manifest
(`crates/workflow-core/src/executor.rs:8205`). Consequently:

1. two callers can both observe no run material;
2. both can complete pure preflight;
3. the first publishes the create-only manifest;
4. the second receives `manifest_exists`, validates identical content, and
   continues; and
5. both may execute the same local check before later binding or event
   persistence resolves the collision.

The store itself correctly exposes create-only manifest publication. The
consumer weakens that ownership boundary by converting an identical
`manifest_exists` result into success.

Required fix:

- make this fresh-run-only path acquire run ownership through create-only
  immutable-manifest publication;
- treat any `manifest_exists` result in this path as
  `executor.authoritative_local_check.existing_run_unsupported`;
- retain the idempotent helper for older paths that intentionally support
  retry or rehydration;
- prove that a losing concurrent or simulated-late claimant fails before
  clock or process use; and
- preserve the documented bounded residue posture for failures after a
  successful claim.

Do not solve this by deleting state, hand-editing the store, adding a
best-effort lock file, or allowing an identical bundle to confer replay
authority.

## 9. Test Quality Assessment

Focused tests adequately cover:

- completed multi-step quiet execution;
- source-bound V2 persistence;
- caller-posture rejection before process use;
- deterministic early-error precedence;
- cross-step monotonicity;
- visible, approval, denial, incomplete, and failed-check failures;
- sequential existing-state rejection;
- create-only bundle residue preventing later reuse;
- V1/V2 serialization compatibility;
- bounded audit projection; and
- `Debug` and error non-leakage.

Missing blocker coverage:

- a deterministic race or late-claim simulation proving that create-only
  run ownership is acquired before process use and cannot be converted into
  idempotent reuse.

The full workspace suite passed with no regression.

## 10. Documentation Assessment

The implementation plan, roadmap, and report accurately describe the selected
check family, aggregate quiet-success semantics, source commitment, privacy
posture, and explicit non-goals.

They must not describe the fresh-run boundary as accepted until the atomic
claim blocker is fixed and re-reviewed.

The latest fresh-pull user review aligns with the intended product direction:
the kernel is coherent and honest, while the next product problem is reducing
low-risk ceremony without losing evidence. This blocker should be fixed
without broadening the phase; it protects exactly that quiet-success path.

## 11. Validation

Completed successfully before and during review:

- authoritative executor focused tests: 5 passed;
- source-binding audit tests: 2 passed;
- source-binding serde compatibility tests: passed;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 12. Non-Blocking Follow-Ups

- Keep the first accepted consumer limited to one explicit `DocsCheck`.
- Plan visible-disclosure continuation only after the fresh-run claim boundary
  is accepted.
- Keep human output concise by default and retain full detail for inspect,
  verbose, JSON, event, audit, and report surfaces.
- Treat the reported Node 24 integration behavior as tooling hardening, not a
  reason to weaken kernel semantics.
- Keep OpenShell as a future optional execution-provider candidate rather than
  coupling it to this local-check blocker fix.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1785028782158604000-2`
- approval:
  `approval/run-1785028782158604000-2/review-scope-approved`
- presentation: `presentation/abfc038702150891`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- validation summary: focused tests, formatting, strict clippy, full workspace
  tests, docs check, and diff check passed
- out-of-kernel work: source inspection, test inspection, review authoring,
  validation commands, and documentation updates
- missing coverage: the kernel coordinated governance only; it did not execute
  engineering checks, edit files, or create a persisted WorkReport artifact

## 14. Recommended Next Phase

Implement one focused atomic fresh-run claim blocker fix, then perform a
focused re-review.

Do not broaden to retry, approval resume, visible-disclosure continuation,
automatic checks, additional check families, reports, CLI, providers,
OpenShell, SideEffects, writes, schemas, hosted behavior, reasoning lineage,
or enterprise administration before that fix is accepted.
