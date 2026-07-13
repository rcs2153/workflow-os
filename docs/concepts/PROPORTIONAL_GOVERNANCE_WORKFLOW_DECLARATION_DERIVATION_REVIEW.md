# Proportional Governance Workflow Declaration Derivation Review

## 1. Executive Verdict

Needs blocker fixes.

The pure derivation boundary is appropriately narrow, deterministic, and
conservative. Static declarations are not mistaken for authority, executed
checks, or write reversibility. One invalidation blocker remains: the
relevant-definition root omits workflow-level retry and escalation policy
definitions even though those policies are referenced by the workflow.

## 2. Scope Verification

The phase stayed within the approved model-only derivation scope. It did not
add filesystem scanning, first-run or CLI integration, executor behavior,
runtime enforcement, persistence, schemas, UI, provider calls, mutations,
automatic approvals, hosted behavior, or release changes.

The executor edit only centralizes existing internal capability-name parsing;
it does not change the capability vocabulary or execution behavior.

## 3. Derivation Assessment

The helper correctly:

- revalidates the supplied project before derivation;
- resolves one workflow step and one version-matching skill;
- derives action class from declared capabilities;
- derives sensitivity from declared approval sensitivity and secret access;
- derives workflow and step-policy minima without treating them as runtime
  proof;
- leaves authority and executed evidence/check posture unknown by default;
- leaves mutation reversibility unknown unless supplied explicitly;
- rejects contradictory SideEffect posture;
- returns only accepted workload-assessment input and does not enforce it.

The conservative legacy behavior for skills without capabilities or adapters
matches the existing executor fallback: local read and local write are assumed
rather than optimistic read-only posture.

## 4. Execution And Disclosure Boundary

The implementation preserves the corrected two-axis model. Visible disclosure
is a disclosure obligation paired with proceeding execution; it is not a
separate blocking execution mode. The helper does not include UI preference in
the assessment input and does not claim that any presentation surface exists.

## 5. Invalidation Assessment

The root uses fixed-width framing and includes the selected workflow hash, step
ID, resolved skill hash, and sorted step-level policy IDs and hashes. Relevant
workflow or skill changes invalidate, and unrelated policy changes do not.

However, `resolve_step_policies` includes only policy references declared on
the step. It does not include `WorkflowDefinition.retry_policy_refs` or
`WorkflowDefinition.escalation_policy_refs`. The helper separately derives
runtime escalation from the presence of workflow escalation references, so
those workflow-level policies are part of the governed definition path. Their
content must be bound into the root.

## 6. Privacy And Error Assessment

Debug output redacts project, workflow, and step identity. Stable derivation
errors do not echo unresolved IDs, paths, definitions, policy contents, or
runtime facts. The helper accepts no raw source, provider payload, command
output, environment value, credential, token, or unbounded report text.

## 7. Test Quality Assessment

Focused tests cover bounded read-only and local-mutation derivation, explicit
unknown runtime facts, compatible overrides, contradiction rejection,
relevant skill invalidation, unrelated-policy stability, and non-leaking
identity errors.

The missing workflow-level-policy invalidation test allowed the blocker to
pass. The blocker fix must add a fixture with a workflow-level referenced
policy and prove that changing its content hash changes the definition root.

## 8. Validation

The following passed during implementation and review:

- `cargo test -p workflow-core --test proportional_governance_workflow_derivation`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

## 9. Blocker

### Workflow-Level Referenced Policies Are Missing From The Definition Root

**Impact:** A workflow-level retry or escalation policy definition can change
without changing the selected step's derived definition root. A future cached
assessment could therefore appear fresh against an incomplete governed
definition set.

**Required fix:** Include workflow-level retry and escalation policy references
in deterministic policy resolution and root hashing. Add focused tests proving
that a referenced workflow-level policy change invalidates while an unrelated
policy change does not. Preserve stable non-leaking resolution errors.

## 10. Non-Blocking Follow-Ups

- Add a validated external-read adapter fixture when the helper is integrated
  into first-run recommendations.
- Decide whether later immutable-run-bundle roots should replace loaded-spec
  hashes before enforcement relies on freshness.
- Keep presentation preferences outside assessment and execution authority.

## 11. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783935718971591000-2`.
- Approval ID:
  `approval/run-1783935718971591000-2/review-scope-approved`.
- Approval presentation: `presentation/143f36793acd6d4d`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Validation summary: focused and full repository validation passed before
  review; docs and diff checks are rerun after this review document.
- Out-of-kernel work: Codex inspected the implementation and authored this
  review; the kernel coordinated governance only.
- Report posture: this document is repository review evidence, not a generated
  or persisted runtime WorkReport artifact.

## 12. Recommended Next Phase

Run a narrow blocker-fix phase for workflow-level referenced-policy
invalidation, then perform focused re-review. Do not start first-run onboarding
integration until the definition root is accepted.

## 13. Fix-Forward Note

The blocker fix now includes workflow-level retry and escalation policy
references in deterministic resolution and root hashing. Focused tests prove
that changing either referenced policy invalidates the root while changing an
unrelated policy does not. The original blocker finding remains preserved here;
acceptance requires a separate focused re-review.
