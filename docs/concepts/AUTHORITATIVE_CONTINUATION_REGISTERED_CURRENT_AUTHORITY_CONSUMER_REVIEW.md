# Authoritative Continuation Registered Current-Authority Consumer Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation is narrow, crate-private, and architecturally aligned with
the accepted plan. The real approval-resume test proves the positive path and
exposed two important pre-existing resume-integrity defects. However, the new
security composition does not yet have direct negative-path and ordering tests
for the guarantees that distinguish it from its individually tested
components.

Fix-forward status: the requested composition-level negative-path, ordering,
replay, and fresh-reassessment tests are now implemented and pass focused
validation. This original verdict is preserved; a separate focused
blocker-fix review must decide whether the boundary is accepted.

## 2. Scope Verification

The implementation stayed within the approved internal proof:

- one private registered-current-authority continuation input;
- one private opt-in executor builder;
- one local current-step skill consumer;
- no public authority API or source configuration;
- no provider mutation, OpenShell integration, nested harness runtime,
  SideEffect execution, schema, SDK, CLI, hosted, or release behavior.

## 3. Composition Assessment

The executor validates the static execution binding before the invoke-policy
event. It then evaluates policy, resolves the local handler, rehydrates the
durable run, validates durable identity, freshly calls the registered source,
derives one private source-backed commitment, projects the exact continuation
brief, consumes the durable first-writer claim, and enters the existing local
skill path.

This ordering matches the plan. The source-backed path cannot silently fall
back to immutable-only continuation, and the default public executor behavior
is unchanged.

## 4. Approval-Resume Assessment

The implementation correctly separates the original execution actor from the
approval decision actor. It also restores the durable immutable bundle onto
the reconstructed resume plan before the selected skill resumes.

These fixes are security-relevant: approval authorizes the pending operation;
it must not rewrite who requested that operation or detach it from its
immutable activation bundle.

## 5. Binding And Commitment Assessment

Static validation checks the exact immutable bundle, workflow, run, step,
execution actor, harness contract ID/version, and contract content hash.
Durable validation also requires `Running` status and exact durable run
identity.

The source capability exposes only one domain-separated internal commitment
covering source snapshot, fact set, assessment, and context consumption. The
executor combines that value with the existing governance commitment before
claiming continuation. No source records or caller-authored readiness value
escape.

## 6. Failure And Privacy Assessment

Errors are stable and bounded. The new code does not serialize source
inventories, grants, context contents, IDs, paths, prompts, command output,
provider payloads, environment values, or credentials. Blocked and source
failure postures are mapped without exposing source-local data.

The implementation retains the existing conservative ambiguity after a claim
whose consumer outcome is not durably known. It does not overclaim external
transactional atomicity or replay prevention.

## 7. Test Assessment

The new tests prove:

- stable payload-free source-backed commitment derivation;
- blocked authority cannot obtain that commitment through the private use
  capability; and
- a real approval-gated immutable-run execution resumes through the new path
  and invokes the local handler exactly once.

The complete repository suite passes, so existing component and regression
coverage remains strong. But the exact composition lacks direct tests for:

- blocked authority and source failure producing zero continuation claims and
  zero handler calls;
- actor, run, step, harness, contract, and immutable-bundle substitution;
- duplicate first-writer and stale-cursor behavior through the source-backed
  path;
- fresh source resolution on a later attempt;
- invoke-policy, claim, hook, attempt, handler, and terminal event ordering;
- unchanged immutable-only and preview behavior adjacent to this opt-in path;
  and
- non-leakage from composition-level negative errors.

The plan explicitly required direct counters or durable-state checks for these
properties. Primitive-level tests do not prove that the executor composed them
in the intended order.

## 8. Blocker

Add focused executor-level negative-path and ordering tests for the new private
composition. The tests must inspect durable events or stores and handler/source
counters rather than only constructors or standalone component outcomes.

No production code redesign is required unless those tests expose a behavioral
defect.

## 9. Non-Blocking Follow-Ups

- Decide the trusted operational source-configuration boundary only after the
  blocker review passes.
- Add bounded continuation-use event/report projection before wider runtime
  exposure.
- Specify crash recovery for claim-without-observed-consumer-outcome.
- Plan authorized execution windows, executor yield, scheduler resumption, and
  typed waits as the next P0 continuity lane rather than representing every
  interruption as approval waiting.

## 10. Validation

- `cargo fmt --all --check`: passed
- `cargo clippy --workspace --all-targets -- -D warnings`: passed
- focused source-backed approval-resume test: passed
- `cargo test --workspace`: passed
- `npm run check`: passed
- `npm run check:integrations`: passed
- `npm run check:docs`: passed
- `git diff --check`: passed before this review record; rerun required

## 11. Governed Review Record

- workflow ID: `dg/review`
- run ID: `run-1786783546831543000-2`
- approval ID:
  `approval/run-1786783546831543000-2/review-scope-approved`
- approval presentation ID: `presentation/7e3cdaf4bd48637e`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- approval proof-use marker: not available; the generic dogfood `approve`
  helper granted the decision even though `phase-start` emitted a distinct
  proof-enforced command. This is a separate P0 dogfood-enforcement defect and
  must not be represented as proof-enforced approval.
- out-of-kernel work: code inspection, test assessment, verdict authorship,
  and validation analysis were performed by the delegated maintainer; the
  kernel governed scope and approval but did not inspect code, edit files,
  execute checks, or mutate git

## 12. Recommended Next Phase

Perform one focused blocker-fix phase for composition-level negative-path and
ordering tests, then repeat this maintainer/security review. Do not proceed to
trusted runtime source configuration, provider mutation broadening, or nested
harness runtime first.
