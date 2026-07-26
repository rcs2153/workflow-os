# Authoritative Local-Check Executor Consumer Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; phase accepted.**

Create-only immutable-manifest publication now owns the fresh-run claim. A
caller that loses that claim fails with the stable
`executor.authoritative_local_check.existing_run_unsupported` error before
clock or local process use, including when its proposed bundle is identical to
the winning bundle.

## 2. Scope Verification

The fix stayed within the approved blocker boundary.

It did not add retry, rehydration, approval resume, cancellation, automatic
checks, additional check families, visible-disclosure continuation,
proportional approvals, reports, artifacts, CLI behavior, schemas, providers,
OpenShell, SideEffects, writes, hosted behavior, reasoning lineage, enterprise
administration, or release changes.

The shared idempotent immutable-bundle helper remains unchanged for existing
paths that intentionally support retry or rehydration.

## 3. Original Blocker

The original consumer checked for existing state before immutable-bundle
publication, then used an idempotent persistence helper. Two concurrent
callers could both pass the initial check, and the losing caller could accept
the winner's identical bundle as success before executing the same local check
again.

That violated the fresh-run-only contract and made the initial ownership check
non-atomic.

## 4. Fix Assessment

The consumer now:

1. rejects existing runtime event history as a fast fail;
2. completes pure bundle, declaration, identity, and reassessment preflight;
3. calls `LocalImmutableRunBundleStore::write_bundle(...)` directly;
4. treats successful create-only manifest publication as the run-to-bundle
   claim;
5. maps every `immutable_run_bundle_store.manifest_exists` result to
   `existing_run_unsupported`; and
6. executes the local process only after the claim succeeds and the claimed
   bundle is reloaded.

`write_manifest_create_only(...)` uses create-new file semantics. Its initial
existence check is only a fast path; a publication race is still resolved by
the create-new operation and mapped to `manifest_exists`.

The selected approach is minimal and uses the store's existing commit marker
instead of adding a lock file, a new store, a transaction protocol, or
best-effort cleanup.

## 5. Fresh-Run Ownership Assessment

The fix establishes one authoritative claimant for a run ID within the local
immutable-bundle store:

- identical content does not confer replay authority;
- content-addressed records written by a losing caller remain harmless
  immutable records;
- only one caller can publish the run-addressed manifest;
- a losing caller cannot reach process execution; and
- no workflow event is appended by the losing caller.

The immutable store and runtime state backend remain separate. The first slice
does not claim a cross-store transaction. If a failure occurs after successful
manifest publication but before events, the bounded immutable residue prevents
run reuse and is disclosed as a limitation.

## 6. Error And Privacy Assessment

The lost-claim path uses a stable static code and message. It does not expose
run IDs, bundle IDs, selected steps, declaration IDs, fingerprints, paths,
commands, process output, source contents, parser payloads, environment
values, provider data, credentials, or tokens.

The fix adds no serialized fields, event kinds, audit payloads, or debug
surfaces.

## 7. Regression Assessment

Existing behavior remains unchanged for:

- the ordinary completed quiet-success path;
- source-bound V2 governance persistence;
- caller-posture and immutable-context preflight;
- failed-check behavior;
- aggregate monotonicity across workflow steps;
- visible, approval-required, denied, and incomplete fail-closed behavior;
- V1/V2 binding compatibility;
- bounded audit projection; and
- existing retry and rehydration paths outside this fresh-run consumer.

## 8. Test Quality Assessment

The focused regression is strong because it separates the two ownership
signals:

- the first caller publishes the bundle and executes one local process;
- the second caller uses a separate empty `LocalStateBackend`;
- both callers use the same immutable store, run ID, and bundle content;
- the second caller receives `existing_run_unsupported`;
- the second backend remains event-free; and
- the process runner call count remains exactly one.

This proves the create-only manifest claim, rather than a prior event-history
check, prevents process reuse.

The broader focused tests continue to cover quiet execution, deterministic
preflight, stricter other-step posture, unsupported aggregate results,
failed-check behavior, source commitment, audit projection, and non-leakage.

## 9. Documentation Assessment

The blocker-fix report accurately describes the corrected ordering, the
create-only claim, the bounded residue posture, and the explicit non-goals.
The implementation plan, original phase report, original review, and roadmap
retain the history of the blocker while linking to this accepted fix.

The current fresh-pull user feedback remains directionally consistent with the
accepted boundary: Workflow OS should reduce low-risk ceremony through
fact-bound quiet success, but should not weaken the evidence and ownership
controls that make quiet execution trustworthy.

## 10. Validation

Completed successfully:

- authoritative local-check executor focused tests: 5 passed;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Keep the accepted consumer explicit, local, and limited to one canonical
  `DocsCheck`.
- Retain create-only residue as a disclosed limitation until a separately
  planned cross-store transaction boundary is justified.
- Continue reducing low-risk ceremony through proportional governance rather
  than by weakening check authority or replay protection.
- Treat OpenShell as a future optional execution-provider candidate, not as
  part of this accepted local-check boundary.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1785031285083358000-2`
- approval:
  `approval/run-1785031285083358000-2/review-scope-approved`
- presentation: `presentation/bde175efa9006d7b`
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

Proceed to the next accepted proportional-governance runtime-composition
boundary after this branch is merged and the roadmap is re-read from current
`main`.

Do not broaden this consumer to retry, approval resume, automatic checks,
additional check families, reports, providers, OpenShell, SideEffects, writes,
schemas, hosted behavior, reasoning lineage, or enterprise administration
without a separately governed phase.
