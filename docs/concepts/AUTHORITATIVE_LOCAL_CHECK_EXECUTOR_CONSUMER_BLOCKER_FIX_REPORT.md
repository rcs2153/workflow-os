# Authoritative Local-Check Executor Consumer Blocker Fix Report

## 1. Executive Summary

The fresh-run ownership blocker in the authoritative local-check executor
consumer is fixed.

The consumer now uses create-only immutable-manifest publication as the
authoritative run-to-bundle claim. Any caller that loses that claim receives
`executor.authoritative_local_check.existing_run_unsupported` before clock or
local process use, even when the competing bundle is byte-for-byte identical.

## 2. Blocker Fixed

Phase review found a time-of-check/time-of-use gap:

- the consumer first checked that no run-bound bundle material existed;
- pure preflight happened afterward;
- bundle persistence used a shared idempotent helper; and
- an identical manifest written by a concurrent caller was accepted as
  success.

That could let two callers execute the same local check under one fresh run
identity before later event or binding persistence resolved the collision.

## 3. Implementation Approach

The fix is intentionally narrow:

- the initial event-history check remains as a fast fail for existing runtime
  state;
- the non-atomic bundle-store emptiness check was removed;
- after all pure preflight succeeds, the consumer calls
  `LocalImmutableRunBundleStore::write_bundle(...)` directly;
- successful create-only manifest publication acquires the claim;
- `immutable_run_bundle_store.manifest_exists` maps to the existing stable
  `executor.authoritative_local_check.existing_run_unsupported` error; and
- all other store failures propagate without process use.

The generic `persist_or_validate_immutable_run_bundle(...)` helper remains
unchanged for older executor paths that intentionally support idempotent retry
or rehydration. It is no longer used by the fresh-run-only consumer.

## 4. Ordering And Failure Boundary

The corrected ordering is:

1. reject existing runtime event history;
2. prepare the plan and evaluate pre-run policy;
3. build the immutable bundle in memory;
4. resolve declarations, derive identities, and complete pure reassessment
   preflight;
5. acquire the run-to-bundle claim through create-only manifest publication;
6. reload the claimed bundle;
7. execute the canonical local check;
8. consume and enforce the source-bound aggregate assessment;
9. persist the governance binding; and
10. append run events and execute the sequential workflow.

A losing claimant cannot convert identical immutable content into replay
authority.

## 5. Test Coverage

Focused coverage now proves:

- the first caller can publish the immutable bundle and execute one check;
- a later claimant using a separate empty state backend and the same run ID and
  bundle receives `existing_run_unsupported`;
- the losing claimant appends no workflow events; and
- the local process call count remains one.

Existing focused tests continue to cover quiet execution, caller-posture
rejection, cross-step monotonicity, unsupported aggregate postures, failed
checks, source binding, and non-leakage.

## 6. Privacy And Error Posture

The fix adds no new serialized values, identifiers, events, audit text, or
debug fields.

The lost-claim error uses the existing stable code and static message. It does
not reveal run IDs, bundle IDs, paths, fingerprints, commands, process output,
source contents, environment values, provider data, credentials, or tokens.

## 7. Scope Explicitly Not Added

This fix does not add:

- retry, rehydration, approval resume, or cancellation support;
- default or automatic checks;
- new check families;
- visible-disclosure continuation;
- proportional approval creation;
- reports, artifacts, evidence attachment, CLI, UI, SDK, or schemas;
- providers, OpenShell, SideEffects, network access, or writes;
- hosted or distributed behavior;
- reasoning lineage;
- enterprise administration; or
- release changes.

## 8. Governed Fix Record

- workflow: `dg/blocker`
- run: `run-1785029021975550000-2`
- approval: `approval/run-1785029021975550000-2/fix-approved`
- presentation: `presentation/82f8c9edfbfcfa74`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- out-of-kernel work: Rust edits, focused tests, repository validation,
  documentation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, or create a persisted WorkReport artifact

## 9. Validation

Completed:

- authoritative local-check executor focused tests: 5 passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 10. Remaining Limitations

- The consumer remains explicit, local, and fresh-run-only.
- The selected family remains one canonical `DocsCheck`.
- The claim spans the immutable bundle store, while runtime events remain in a
  separately supplied state backend. The first slice does not introduce a
  cross-store transaction.
- Failures after a successful claim may leave an immutable bundle without run
  events. That bounded residue remains intentional and prevents reuse.
- Visible disclosure, proportional approval, retry, resume, report, and CLI
  behavior remain absent.

## 11. Recommended Next Phase

Perform a focused blocker-fix review.

Do not broaden the consumer until the create-only claim semantics and
regression coverage are accepted.
