# Scoped Capability GitHub Read-Only Integration Blocker Fix Report

## 1. Executive Summary

The scoped GitHub repository-metadata read now uses Workflow OS's validated
current-authority snapshot contract instead of an ad hoc selected-record hash.
It also has an explicit source-continuity posture that rejects generation
rollback and same-generation watermark substitution before the adapter call.

The helper remains crate-private, opt-in, read-only, and disconnected from the
executor, workflow schemas, CLI, persistence, and provider mutation paths.

## 2. Blockers Fixed

The prior review found two source-integrity gaps:

1. the provider path did not construct a validated exact-query
   `CurrentAuthoritySourceSnapshot`; and
2. the call had no prior trusted source state against which rollback could be
   detected.

Both are fixed in the bounded integration path.

## 3. Snapshot Implementation

The helper now constructs a committed source request for exactly one
capability/resource query and exactly two fact families:

- capability grants; and
- capability availability.

It constructs the snapshot through the existing committed snapshot model with:

- the registered source commitment;
- one exact query;
- atomic-snapshot consistency;
- `CompleteForExactQuery` completeness;
- exact family counts;
- the registered observation read window and validity bound;
- an inventory-bound watermark;
- the source generation; and
- a bounded records commitment.

The snapshot model validates family coverage, target coverage, consistency,
time posture, freshness, and commitment consistency. Its validated snapshot
commitment now binds the capability-use fact set and receipt.

## 4. Continuity And Rollback Boundary

`CapabilityUseSourceContinuity` is private operation input vocabulary with two
postures:

- `FirstObservation` explicitly states that no prior trusted source
  observation is available; and
- `PriorObservation` carries a prior trusted generation and watermark.

For prior observations, the helper rejects:

- a current source without comparable generation;
- a current generation below the prior generation; and
- a changed watermark at the same generation.

A higher generation is allowed to carry a new watermark. This helper does not
persist continuity state; a future Core-owned caller must retain and supply the
trusted prior observation.

## 5. Failure Ordering

Snapshot and continuity validation occur before capability resolution,
projection, adapter-request construction, and provider invocation. Any
snapshot, freshness, rollback, or substitution failure returns
`BlockedBeforeRead` and makes zero client calls.

Existing read-failure and post-read receipt-failure semantics are unchanged.

## 6. Privacy And Redaction

The continuity enum Debug implementation redacts the watermark. New errors use
stable codes and bounded messages without source facts, repository values,
credentials, paths, payloads, or watermark values. Snapshot and receipt
evidence remain commitment-based.

## 7. Test Coverage

The focused suite now has 16 tests. Added regressions prove:

- incomplete exact-query availability coverage blocks with zero calls;
- a matching prior generation and watermark permits the bounded read;
- generation rollback blocks with zero calls;
- same-generation watermark substitution blocks with zero calls;
- wrong run, step, and harness identity each block with zero calls; and
- invalid owner/repository boundaries block with zero calls.

The existing tests continue to cover canonical success, policy provenance,
broad grants, stale sources, unavailable capability, independent
prerequisites, sensitivity, adapter failure, post-read receipt failure,
unverified claims, and secret-like input non-leakage.

## 8. Scope Explicitly Not Added

This fix adds no public API, automatic adapter gating, executor integration,
workflow configuration, schema, SDK, CLI behavior, receipt persistence, live
network requirement, second provider operation, provider mutation, hosted
identity, nested harness behavior, cryptographic proof, or release change.

## 9. Validation

- Focused GitHub authority integration tests: passed, 16 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Governed Phase Record

- workflow ID: `dg/blocker`
- run ID: `run-1792695660517682000-2`
- approval ID: `approval/run-1792695660517682000-2/fix-approved`
- presentation ID: `presentation/cf8972654c5a3562`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations; the
  ordered trail includes the approval request/grant, policy decisions, six
  scheduled and successful governed steps, and terminal completion
- approval-presentation enforcement: persisted proof enforced
- kernel boundary: governance coordination only; implementation, tests,
  documentation, validation, Git, pull-request, CI, and merge work remain
  outside the kernel

## 11. Remaining Limitations

- The caller must retain trusted continuity state between observations.
- `FirstObservation` cannot detect history that the caller does not possess;
  its no-history posture is explicit rather than inferred.
- The integration remains private and fixture-tested.
- Existing direct adapter APIs remain unchanged and are not automatically
  capability-gated.

## 12. Recommended Next Phase

Perform a focused blocker-fix review. Verify that the operation now executes
the existing exact-query snapshot invariants, that continuity comparison is
truthful, that all denial paths make zero provider calls, and that no public or
write boundary broadened. Do not add another operation before that review.
