# Scoped Capability GitHub Read-Only Integration Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed with non-blocking follow-ups.**

The bounded GitHub repository-metadata read now constructs and validates an
exact-query `CurrentAuthoritySourceSnapshot`, binds its commitment into the
fact set and successful-use receipt, and rejects comparable source rollback or
same-generation watermark substitution before provider invocation. The prior
ad hoc snapshot hash is gone.

The implementation remains crate-private, opt-in, injected-client, and
read-only. It is accepted at that boundary. This verdict does not authorize a
second operation, executor or workflow-schema wiring, automatic adapter
gating, provider mutation, or public continuity API.

## 2. Scope Verification

The fix stayed within the approved blocker scope. It changed the private
source assessment and tests plus truthful roadmap/report documentation. It
added no public API, executor integration, workflow configuration, schema,
SDK, CLI behavior, persistence, live-network acceptance requirement, provider
mutation, hosted identity, nested harness execution, or release change.

Existing direct GitHub adapter APIs remain unchanged and are not represented
as capability-gated.

## 3. Original Blockers

The implementation review identified two blockers:

1. the provider path committed an ad hoc filtered-record hash instead of
   executing the existing source-snapshot invariants; and
2. the operation had no explicit prior trusted generation or watermark with
   which to detect rollback.

Both defects are addressed for the private integration path.

## 4. Exact-Query Snapshot Assessment

The operation now constructs a dedicated request commitment for one exact
capability/resource query and exactly two fact families: capability grants
and capability availability. It calls
`CurrentAuthoritySourceSnapshot::new_from_committed_request` with:

- the registered source commitment;
- one exact query;
- atomic-snapshot consistency;
- `CompleteForExactQuery` completeness;
- exact returned-family and fact-count posture;
- the registered read window and validity bound;
- the source generation;
- an inventory-bound watermark; and
- a bounded records commitment.

The shared snapshot constructor validates registration and consistency,
family coverage, exact availability target coverage, read-window posture,
freshness, validity bounds, fact-count bounds, and commitment consistency. The
validated snapshot commitment now binds the capability-use fact set and the
eventual successful-use receipt. This fixes the first blocker.

The source derives the selected records from its complete private registered
inventories. Callers still cannot inject grants, availability records, fact
sets, projections, snapshots, or receipts as authority.

## 5. Continuity And Rollback Assessment

`CapabilityUseSourceContinuity` makes the source-history posture explicit:

- `FirstObservation` states that no prior trusted observation is available;
  and
- `PriorObservation` supplies a trusted generation and watermark for
  comparison.

When prior continuity is supplied, the operation rejects a missing comparable
current generation, a lower current generation, and a changed watermark at
the same generation. These checks occur before capability resolution,
projection, request construction, or adapter invocation. A higher generation
may carry a changed watermark.

This is truthful for the current crate-private slice. `FirstObservation`
cannot detect history the caller does not possess, and the helper does not
persist continuity. Therefore, any future executor-facing or public adoption
must obtain prior continuity from Core-owned durable state and must not let an
untrusted caller downgrade an established source to `FirstObservation`. That
is a required follow-up boundary, not a claim provided by this helper today.

## 6. Failure Ordering Assessment

Snapshot construction and continuity validation are inside the existing
linear fail-closed preparation pipeline and precede the adapter call. The new
incomplete-coverage, generation-rollback, watermark-substitution, execution-
identity, and repository-identity tests all prove zero client calls on denial.

The existing outcome domains remain truthful:

- `BlockedBeforeRead` means no provider call occurred;
- `ReadFailed` means the provider call failed and no receipt exists;
- `Succeeded` carries the bounded read outcome, projection, and receipt; and
- `ReadSucceededReceiptFailed` preserves successful provider-call truth while
  reporting missing receipt evidence.

## 7. Privacy And Redaction Assessment

Continuity Debug output redacts the watermark. New validation errors use
stable bounded codes and do not echo source facts, repository input,
credentials, paths, provider payloads, or watermark values. Snapshot, fact-set,
assessment, resource, and outcome evidence remain commitment-based.

No raw GitHub body, authorization header, environment value, grant payload, or
source inventory is copied into receipt evidence or diagnostics.

## 8. Regression And Test Assessment

The focused suite has 16 tests and covers:

- the canonical successful read;
- policy-precheck provenance;
- exact run, step, and harness scope;
- validated exact-query availability coverage;
- matching prior continuity;
- generation rollback;
- same-generation watermark substitution;
- owner and repository input boundaries;
- stale and unavailable authority;
- prerequisite-bearing grants and sensitivity ceilings;
- adapter failure and post-read receipt failure;
- unverified receipt deserialization; and
- secret-like input non-leakage.

The full workspace format, lint, test, documentation, integration, adapter,
runtime, persistence, and report suites pass. The accepted behavior is tested
without requiring live GitHub access.

## 9. Blockers

None for the current crate-private, opt-in, read-only integration slice.

## 10. Non-Blocking Follow-Ups

- Before executor-facing or public adoption, persist source continuity in a
  Core-owned store and derive first/prior observation posture from that state.
- Add a direct regression for the prior-observation/current-generation-missing
  branch and for higher-generation watermark advancement.
- Remove the duplicate local `calls` binding in the repository-boundary test
  during a future touched-file change; it has no behavioral effect.
- Keep direct adapter APIs explicitly outside the capability-gated claim.
- Do not generalize this private query shape into a public query framework
  without separate planning and compatibility review.

## 11. Recommended Next Phase

Close the scoped capability GitHub repository-metadata vertical slice as
accepted. Do not add a second operation or provider mutation by implication.
Reconcile the authoritative Active Delivery Queue and select the next
executable no-write or hardening milestone, while the dedicated draft-pull-
request live sandbox proof remains dependency-tracked until its credential is
available.

## 12. Governed Phase Record

- workflow ID: `dg/review`
- run ID: `run-1792696719784529000-2`
- approval ID:
  `approval/run-1792696719784529000-2/review-scope-approved`
- presentation ID: `presentation/bf3a53ae00828a1f`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations; the
  ordered trail includes the approval request/grant, eight policy decisions,
  six scheduled and successful governed steps, and terminal completion
- approval-presentation enforcement: proof enforced with one matching
  persisted presentation record and event marker
- validation summary: `cargo fmt --all --check`, workspace Clippy with warnings
  denied, `cargo test --workspace`, `npm run check:docs`, and
  `git diff --check` passed
- kernel boundary: governance coordination only; source inspection, review
  authorship, validation, Git, pull-request, CI, and merge work remain outside
  the kernel
