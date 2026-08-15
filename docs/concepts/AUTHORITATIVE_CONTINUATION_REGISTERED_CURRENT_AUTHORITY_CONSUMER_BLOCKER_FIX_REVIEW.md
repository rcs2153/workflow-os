# Authoritative Continuation Registered Current-Authority Consumer Blocker Fix Review

## 1. Executive Verdict

**Needs additional blocker fixes.**

The blocker fix materially improves the security evidence and the complete
repository suite is green. It directly proves failure-before-claim,
failure-before-handler, claim-before-handler ordering, event ordering,
contract-substitution privacy, and later source reassessment. However, two
parts of the original blocker remain tested below or before the exact composed
boundary rather than through it.

## 2. Scope Verification

The fix stayed within test and documentation scope. It added no production
API, authority-source configuration, provider mutation, OpenShell integration,
nested harness runtime, SideEffect execution, schema, SDK, CLI, hosted, or
release behavior. The only production edits remain those reviewed in the
original implementation phase.

## 3. Direct Composition Evidence

Accepted direct evidence now proves:

- blocked authority produces zero continuation claims and zero handler calls;
- stale source posture produces zero claims and zero handler calls;
- one substituted harness contract fails closed without leaking either
  contract identifier;
- the durable continuation claim exists before local handler entry;
- approval, resume, invoke policy, invocation request, attempt, success, and
  terminal events retain their required order; and
- the same registered source is reassessed at a later Core-selected use time.

These tests use a real approval-gated immutable run, durable local state,
ordered workflow events, and a registered handler counter. They prove
composition behavior rather than construction alone.

## 4. Remaining Blocker Evidence

The terminal replay test calls approval again after the run is complete. It is
rejected before fresh current-authority resolution and before the continuation
claim boundary. Therefore it does not prove the planned duplicate first-writer
or stale-cursor behavior through the source-backed composition.

The contract-substitution test directly composes one mismatch, but actor, run,
step, harness version, and immutable-bundle substitutions still rely on the
static validator and lower-level cursor tests. The original review explicitly
required those identities at the exact executor boundary.

Add focused test-only seams or fixtures that:

1. preload or race the exact continuation claim while leaving the durable run
   eligible for fresh authority resolution, then prove duplicate posture and
   zero handler calls;
2. advance the durable cursor after claim selection in a controlled test path,
   then prove stale-cursor failure and zero handler calls; and
3. table-drive actor, run, step, harness ID/version, contract hash, and
   immutable-bundle substitutions through the composed executor entry.

The seam must remain test-only or crate-private and must not create a public
way to forge runtime state.

## 5. Privacy And Failure Assessment

The added tests and fixture remain payload-free. Errors use stable codes and
do not echo source records, context targets, substituted contract identifiers,
paths, prompts, command output, provider payloads, environment values, or
credentials. Structured JSON is used for the test-only durable claim check.

## 6. Regression Assessment

All existing public CLI, executor, approval, adapter, provider-write,
WorkReport, state-backend, hosted, and documentation checks pass. Default and
immutable-only executor behavior remain unchanged.

## 7. Validation

- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- focused source-backed executor tests: passed, 5 tests
- focused same-source reassessment test: passed, 1 test
- `cargo test --workspace`: passed
- `npm run check`: passed
- `npm run check:integrations`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed before this review record; rerun required

## 8. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1786790188047752000-2`
- approval ID:
  `approval/run-1786790188047752000-2/review-scope-approved`
- approval presentation ID: `presentation/cfa9d400c347c7c0`
- approval outcome: granted under delegated-maintainer authority through the
  exact proof-enforced approval command
- out-of-kernel work: code inspection, test assessment, verdict authorship,
  and validation analysis were performed by the delegated maintainer; the
  kernel governed scope and approval but did not inspect code, edit files,
  execute checks, or mutate git

## 9. Recommended Next Phase

Perform one final focused blocker-fix phase for composed duplicate/stale claim
outcomes and the remaining exact binding substitutions, then repeat focused
maintainer/security review. Do not broaden runtime source configuration,
provider mutations, or nested harness execution first.

## Fix-Forward Note

The requested final blocker fix was implemented and independently accepted in
[Authoritative Continuation Registered Current-Authority Consumer Final
Blocker Fix
Review](AUTHORITATIVE_CONTINUATION_REGISTERED_CURRENT_AUTHORITY_CONSUMER_FINAL_BLOCKER_FIX_REVIEW.md).
This note preserves the original blocker verdict while recording its resolved
status.
