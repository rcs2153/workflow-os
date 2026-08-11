# Proportional-Governance Selected Local Project Consumer Review

## 1. Executive Verdict

Needs blocker fixes.

The additive composition is otherwise narrow, coherent, and well tested, but
the selected consumer still accepts its governance evaluation time from the
public caller. That is incompatible with the planned Core-owned trust boundary
and must be fixed before merge or CLI adoption.

## 2. Scope Verification

The implementation stayed within the selected-consumer composition scope. It
added no CLI adoption, executor default, provider execution, OpenShell
integration, SideEffect execution, schema, example, hosted behavior, new
mutation family, or release change.

## 3. Composition Assessment

The route owns the fixed runtime-fact source registration and derives the
selected evidence/check fact from the actual canonical project-validation
result. The approval path requires persisted presentation proof, rebuilds and
compares the current immutable project bundle, reruns the canonical check, and
reproduces the durable assessment core and source registration before grant
mutation. It then reuses the accepted authority-receipt and report-artifact
closure.

Distinct aggregate-governance and workflow step approvals remain distinct. A
first grant can pause at the workflow step approval without creating a terminal
artifact; the second proven grant completes the workflow and closure.

## 4. Blocking Finding

`LocalSelectedProjectValidationGovernanceRequest` and
`LocalSelectedProjectValidationArtifactDecisionInput` expose `evaluated_at` as
public caller-authored input. The Core-owned runtime-fact source then sets its
observation time from that evaluation time. Consequently, the selected product
consumer still lets its caller choose the clock used by source freshness
evaluation.

This contradicts the accepted plan, which identifies caller-provided
evaluation time as part of the unsafe generic boundary and requires source
identity, registration, fact derivation, and freshness behavior to be owned by
Core for the selected consumer.

The blocker fix must:

- remove caller-provided evaluation time from both selected-consumer public
  request types;
- obtain a fresh Core-owned time independently for initial routing and each
  granted decision call;
- preserve source-free denial after proof validation;
- preserve deterministic testability without exposing a public authority clock;
- prove that callers cannot backdate or future-date the selected assessment;
  and
- keep the generic registered-source APIs unchanged.

## 5. Approval And Failure Ordering

Except for the caller-controlled clock, ordering is correct:

- presentation proof validation precedes decision-time check execution;
- changed relevant definitions fail immutable-bundle comparison before recheck;
- failed checks and reassessment mismatch precede approval mutation;
- denial invokes neither the canonical check nor runtime-fact source;
- trusted receipt derivation follows only a successful grant;
- report and persistence failures preserve truthful terminal decision state;
  and
- no fake evidence or fabricated identifier is introduced.

## 6. Privacy And Compatibility

Manual Debug implementations redact selected identities, source details,
report inputs, and evaluation metadata. Errors remain stable and bounded. The
new API does not copy raw check output, source content, paths, environment
values, provider payloads, or credentials. Existing executor and CLI APIs are
unchanged.

## 7. Test Assessment

The five focused tests cover complete two-gate success, denial without recheck
or writes, missing presentation proof, relevant-definition invalidation, and
failed decision-time validation. The workspace regression suite passes.

The missing blocker-level test is structural and behavioral proof that the
selected consumer has no caller-authored evaluation-time surface and uses a
fresh Core-owned evaluation time at each assessment boundary.

## 8. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1786443542778305000-2`.
- Approval ID: `approval/run-1786443542778305000-2/review-scope-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/9305ac013b7a8ede`.
- Terminal status: `Completed`.
- Event summary: 39 events, one approval request, one approval grant, six
  successful skill invocations, no retries, and no escalations.
- Review work was performed by the delegated maintainer outside the kernel;
  the kernel governed scope and approval and retained the durable event trail.

## 9. Validation Reviewed

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Focused selected-consumer tests: 5 passed.
- `cargo test --workspace`: passed; opt-in live tests remained ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Blockers

1. Remove caller control of selected-consumer evaluation time and use a
   Core-owned fresh clock at initial route and granted decision boundaries.

## 11. Non-Blocking Follow-Ups

- Keep CLI adoption separate after blocker-fix review.
- Keep generic source and evaluation-time inputs available only on the existing
  explicit generic APIs.
- Preserve the two-gate behavior when both aggregate and step approvals exist.

## 12. Recommended Next Phase

Execute a focused blocker fix for Core-owned evaluation time, then perform a
blocker-fix review before merging PR 458. Do not begin CLI adoption first.

## 13. Fix-Forward Status

The original blocker finding above remains the authoritative review record. A
subsequent focused fix removed `evaluated_at` from both selected-consumer public
request types. Initial route assessment and every decision call now select a
fresh timestamp inside Core, so a selected-consumer caller cannot backdate or
future-date the source observation or freshness evaluation. Generic
registered-source APIs remain unchanged.

The fix preserves presentation-proof-first ordering and the source-free denial
path. Its separate blocker-fix review accepts the correction. This original
review remains unchanged above as the durable record of why the fix was
required.
