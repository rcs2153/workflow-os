# Proportional Governance Workflow Declaration Derivation Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to one explicit read-only onboarding recommendation
integration.

## 2. Review Scope

This focused re-review inspected only the workflow-level referenced-policy
invalidation fix, its regression tests, the completed repository validation,
and the associated phase documentation. It did not authorize runtime
enforcement, persistence, schema changes, UI, provider mutation expansion, or
additional governance model families.

## 3. Original Blocker

The initial implementation included step-level policy definitions in the
relevant-definition root but omitted policies referenced by
`WorkflowDefinition.retry_policy_refs` and
`WorkflowDefinition.escalation_policy_refs`. A future cached assessment could
therefore remain apparently fresh after a governing workflow-level policy
changed.

## 4. Fix Assessment

The fix resolves workflow-level retry and escalation references through the
same deterministic, deduplicated `PolicyId` set used for step-level policy
references. The existing stable resolution error and sorted root framing are
reused. No public model, runtime behavior, persistence path, or schema was
added.

The relevant-definition root now binds:

- the selected workflow content hash;
- the selected step identity;
- the resolved skill content hash;
- step requirement, approval, retry, and escalation policy hashes;
- workflow-level retry and escalation policy hashes.

Unrelated policy definitions remain outside the root. This is the correct
bounded invalidation posture for the current helper.

## 5. Test Assessment

The focused regression test now proves that changing either a workflow-level
retry policy or workflow-level escalation policy changes the root. It also
preserves the existing proof that an unrelated policy change does not change
the root. The broader derivation tests continue to cover conservative unknown
runtime facts, action and sensitivity derivation, SideEffect contradiction
rejection, and non-leaking errors and Debug output.

## 6. Privacy And Compatibility

The fix adds no raw definition contents, source paths, provider payloads,
command output, credentials, or free-form report text. Errors remain stable and
do not echo workflow, step, skill, or policy identifiers. Existing executor
capability parsing is centralized without changing its vocabulary or fallback
behavior.

## 7. Validation

The following passed after the blocker fix:

- `cargo test -p workflow-core --test proportional_governance_workflow_derivation`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

## 8. Blockers

None.

## 9. Non-Blocking Follow-Ups

- A later immutable-run-bundle root may replace loaded-spec hashes before
  runtime enforcement relies on freshness.
- The first onboarding integration should add an external-read adapter fixture
  if that recommendation path needs to distinguish read-only adapter access.
- Presentation preferences must remain separate from execution disposition and
  required disclosure.

## 10. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783937755564203000-2`.
- Approval ID:
  `approval/run-1783937755564203000-2/review-scope-approved`.
- Approval presentation: `presentation/08bb52bd9d53b876`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Validation summary: focused and full repository validation passed before
  this review; docs and diff checks are rerun after review edits.
- Out-of-kernel work: Codex inspected the fix and authored this review; the
  kernel coordinated governance only.
- Report posture: this document is repository review evidence, not a generated
  or persisted runtime WorkReport artifact.

## 11. Recommended Next Phase

Integrate the accepted derivation into one explicit, read-only first-run
recommendation path. The path should infer bounded governance posture from
validated declarations and safe metadata, expose unresolved facts, and remain
review-only. It must not enforce decisions, fabricate authority, persist
assessments, broaden provider mutations, or make UI visibility an execution
mode.
