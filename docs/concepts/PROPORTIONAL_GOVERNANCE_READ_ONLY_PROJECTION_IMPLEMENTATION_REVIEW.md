# Proportional Governance Read-Only Projection Implementation Review

## 1. Executive Verdict

**Needs blocker fixes.**

The model, mapping, source-of-truth boundary, and scope are correct. The
implementation is not yet acceptable because unknown caller-supplied enum values
can be echoed by derived Serde errors before the projection's fixed non-leaking
validation error is reached.

## 2. Scope Verification

The phase stayed within the approved model/helper scope. It introduced no
executor behavior, quiet-success activation, approval-default change,
automatic approval, persistence, workflow event, CLI, schema, report, provider
call, provider write, mutation expansion, hosted behavior, RBAC, example, or
release change.

## 3. Model Assessment

The projection is appropriately minimal and private-fielded. It contains only:

- accepted interaction mode;
- accepted risk class;
- stable reason codes;
- operator action requirement;
- assessed-not-enforced posture;
- not-persisted posture.

The helper consumes an accepted `ProportionalGovernanceDecision` and does not
recompute risk. Accessors are read-only and no unstable contextual IDs or
free-form summaries were added.

## 4. Deterministic Mapping Assessment

The mapping is exhaustive and correct:

- quiet capture to no action;
- visible disclosure to review;
- blocking approval to approval;
- denial to denied.

The projection preserves source mode, risk class, reasons, and reason ordering.
It cannot mutate or downgrade the source decision.

## 5. Validation And Serde Assessment

The shared consistency validator correctly rejects mismatched mode/risk values
and decisions without the profile-minimum reason. Projection deserialization
also rejects a mismatched action requirement and unsupported projection posture.

However, the private deserialization wire uses public enums that derive
`Deserialize`. Serde processes those fields before the projection's custom
consistency check. An unknown value can therefore produce an error such as an
unknown-variant message containing the supplied value. The same exposure exists
for an unknown reason element.

This violates the accepted plan requirement that deserialization errors not
echo raw serialized values or secret-like metadata.

## 6. Privacy And Redaction Assessment

Valid projections are payload-free and derived `Debug` is safe because all
stored values are closed enums. Serialization contains no contextual IDs,
paths, payloads, output, timestamps, or credentials.

The blocker is limited to invalid deserialization errors. Valid storage and
serialization posture is otherwise accepted.

## 7. Test Quality Assessment

Existing tests adequately cover:

- all mode/action mappings;
- source decision preservation;
- source non-mutation;
- valid serde round trip;
- inconsistent action rejection;
- inconsistent mode/risk rejection;
- payload-free `Debug` and serialization;
- existing selector regressions.

Missing blocker tests:

- unknown mode does not echo the supplied value;
- unknown risk class does not echo the supplied value;
- unknown action requirement does not echo the supplied value;
- unknown decision posture does not echo the supplied value;
- unknown persistence posture does not echo the supplied value;
- unknown reason does not echo the supplied value.

## 8. Compatibility Assessment

The new API is additive and does not affect existing callers. The flat
serialization shape is suitable for a preview model contract, provided the
invalid-input error boundary is fixed before acceptance. No workflow schema or
CLI compatibility commitment was introduced.

## 9. Relationship To Runtime

The implementation correctly remains assessed rather than enforced and in
memory rather than persisted. It does not create approvals, continue runs,
deny runs, append events, or claim a WorkReport exists.

## 10. Blockers

1. Replace the projection wire's directly derived enum deserialization with a
   bounded projection-specific parsing boundary that returns fixed errors and
   never echoes unknown values, including reason elements.
2. Add focused non-leakage regression tests for every projection enum field and
   unknown reason input.

The fix must not remove validated deserialization, alter the public enum
vocabulary, or redesign the core selector.

## 11. Non-Blocking Follow-Ups

- Keep the projection serialization shape under preview compatibility review
  before any schema or CLI exposure.
- Plan immutable run-bundle hardening after the blocker fix is accepted.

## 12. Recommended Next Phase

Perform a bounded blocker fix for projection deserialization error redaction,
run the full validation suite, and conduct a focused blocker-fix review.

Do not begin executor integration, quiet-success activation, persistence, CLI,
or additional provider mutation work.

## 13. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 14. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783821387941387000-2`.
- Approval ID:
  `approval/run-1783821387941387000-2/review-scope-approved`.
- Presentation ID: `presentation/fc4ddf3151577a99`.
- Approval outcome: granted through the proof-enforced approval path.
- Final status: completed.
- Event summary: 39 events, one approval request, one proof-marked approval
  grant, eight policy decisions, six scheduled steps, six successful mock skill
  invocations, no retries, and no escalations.
- Out-of-kernel work: Codex inspected the model, helper, exports, tests, docs,
  and serde boundary and authored this review. The kernel governed scope and
  approval but did not perform the review or edit files.
- Report posture: this document is the review record. No runtime WorkReport or
  report artifact was generated or persisted.
