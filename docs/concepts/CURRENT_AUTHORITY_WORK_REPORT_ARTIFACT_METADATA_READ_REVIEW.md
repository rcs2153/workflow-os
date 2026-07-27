# Current-Authority WorkReport Artifact Metadata Read Review

## 1. Executive Verdict

Phase accepted; keep the metadata-read operation private and return to the
active proportional-governance and quiet-success roadmap lane.

The implementation proves one useful Core-owned current-authority consumer
without creating reusable authority, generic context dereference, report-body
access, or runtime execution authority. No blocker remains.

## 2. Scope Verification

The phase stayed within the approved private read-only scope.

It did not add public APIs, report-body access, executor integration, provider
or OpenShell behavior, sandbox execution, SideEffects, writes, event append
behavior, persistence changes, schemas, SDKs, CLI behavior, examples,
dependencies, hosted behavior, or release changes.

## 3. Exact-Target And Contract Assessment

The input is pinned to one immutable execution binding, one matching required
context contract, and one exact `WorkReportId`. Contract construction already
rejects duplicate targets, so target lookup cannot select among conflicting
requirements.

The operation requires the target to be:

- `GovernedContextReferenceTarget::WorkReport`;
- declared at `GovernedContextAccessLevel::BoundedMetadata`; and
- governed by `RequiredContextObligation::Required`.

Reference-only, optional, and undeclared targets fail before the store is
reachable. Contract identity, version, and content hash must match the
execution binding.

## 4. Same-Call Authority Assessment

The concrete read reuses the private `use_current_authority` boundary.
Registered-source selection, capability resolution, governed-context
projection, and exact contract consumption rerun for every call. The consumer
closure is invoked only when that assessment is `Ready`.

The borrowed capability is neither cloneable nor serializable and does not
escape the same call. The store is passed explicitly to the concrete operation;
the capability itself does not retain or expose it.

## 5. Store-Access And Outcome Assessment

The operation performs at most one exact
`read_work_report_artifact(run_id, report_id)` call. It does not list or write
artifacts.

The result reconciliation is fail-closed:

- ready authority plus a valid exact record returns `Found`;
- ready authority plus no record returns `NotFound`;
- blocked authority returns `Blocked` with zero reads;
- source failure returns `SourceFailure` with zero reads;
- a read, validation, identity, or sensitivity failure returns bounded
  `StoreFailure`; and
- disagreement between the captured read and use posture returns a stable
  internal consistency error.

No failed or missing read fabricates metadata or evidence.

## 6. Identity And Sensitivity Assessment

The returned artifact is revalidated even though the store contract already
requires validated records. Its report ID and run ID must match the exact
request and immutable execution binding.

Artifact sensitivity must not exceed either:

- the matching required-context requirement ceiling; or
- the immutable execution binding ceiling.

The earlier context-resolution path independently blocks a declared reference
whose sensitivity exceeds its contract. The concrete read therefore preserves
both reference-time and artifact-time sensitivity checks.

## 7. Privacy And Visibility Assessment

The private view exposes only:

- report ID;
- run ID;
- terminal run status; and
- sensitivity.

It does not expose the contained `WorkReport`, sections, citations, summaries,
notes, risks, limitations, disclosures, hashes, paths, commands, logs,
provider payloads, credentials, or redaction details.

The input, view, and outcome are `pub(super)` inside a private crate module and
are not exported from `workflow-core`. The view and outcome are not
serializable. Debug output redacts both identifiers and underlying store errors
are discarded.

## 8. Test Quality Assessment

The focused registered-source suite contains 34 tests and directly proves:

- one exact read for a ready target;
- zero reads for reference-only, optional, undeclared, revoked, unresolved,
  stale, changed-run, and sensitivity-blocked paths;
- explicit not-found and bounded store-failure outcomes;
- artifact identity and actual-sensitivity rejection after one read;
- fresh authority resolution and one read on each repeated call;
- no store writes or list calls; and
- Debug and error non-leakage.

The full workspace suite also exercises the existing required-context,
capability, WorkReport, artifact-store, runtime, provider, SideEffect, and
proportional-governance boundaries.

## 9. Documentation Assessment

The roadmap, accepted plan, and implementation report accurately describe the
operation as private, exact-target, metadata-only, same-call, and read-only.
They do not claim durable replay prevention, transactional source/store
consistency, public consumption, executor wiring, provider execution, or
writes.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Any future public or executor consumer requires a separate compatibility,
  privacy, and runtime-semantics review.
- Production use still needs durable replay prevention and explicit
  source/store consistency semantics.
- A later consumer must remain concrete and target-specific rather than
  exposing the private generic authority callback.
- No additional governed-context target family should be added without a
  concrete product need and its own bounded review.

## 12. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1785184773388951000-2`
- approval ID:
  `approval/run-1785184773388951000-2/review-scope-approved`
- approval-presentation ID: `presentation/1c94e0e731635d38`
- approval-presentation content hash:
  `1c94e0e731635d388706a3f4eb55a8ea89b8cfdf11d15f5f5a87bc8c7284047a`
- approval outcome: granted under delegated-maintainer authority
- approval-presentation enforcement: proof persisted before approval
- governed status: completed
- out-of-kernel work: the delegated maintainer inspected code and tests,
  authored this review, updated phase documentation, and ran validation; the
  kernel governed scope and approval but did not read the artifact store,
  edit files, execute checks, or mutate git

## 13. Validation

- focused registered-source tests: passed, 34 tests
- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- `cargo test --workspace`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed

## 14. Recommended Next Phase

Do not broaden this operation into a public API or executor consumer.

Return to the active proportional-governance and quiet-success roadmap lane.
Fresh user evaluation confirms that the next product problem is reducing
ceremony for low-risk work while preserving evidence, event history,
disclosure, and report posture. That work should consume the already accepted
governance models rather than add another authority primitive or mutation
family.
