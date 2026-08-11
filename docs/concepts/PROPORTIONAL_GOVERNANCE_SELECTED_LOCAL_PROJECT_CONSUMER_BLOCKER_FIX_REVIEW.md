# Proportional-Governance Selected Local Project Consumer Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; merge the selected-consumer composition and proceed to separate
CLI adoption planning.

## 2. Scope Verification

The fix stayed within the approved authority-clock boundary. It removed two
public timestamp fields, selected fresh timestamps inside the existing Core
composition functions, updated focused tests to use the reduced API, and
corrected the phase documentation.

It added no CLI adoption, executor default, provider or OpenShell integration,
SideEffect execution, schema, example, hosted behavior, new mutation family,
unrelated refactor, or release posture change.

## 3. Original Blocker Restatement

The selected route and approval-artifact request types exposed
`evaluated_at`. The fixed source used that value as both the evaluation and
observation time, so a public caller could backdate or future-date the
freshness boundary of a product path documented as Core-owned.

## 4. Fix Assessment

`LocalSelectedProjectValidationGovernanceRequest` now contains only the closed
Core-owned execution request.
`LocalSelectedProjectValidationArtifactDecisionInput` contains approval,
execution, report, and selected artifact-gate inputs but no evaluation time.

`route_selected_project_validation_governance` selects one fresh timestamp
inside Core for initial assessment. The granted decision path selects a new
timestamp only after approval-presentation proof succeeds and before current
bundle reassessment, canonical project validation, and runtime-fact source
construction. Each decision call therefore receives a new Core-selected time.

The approach is minimal and idiomatic. It does not introduce a public clock,
new trait, hidden global configuration, or alternate selected path.

## 5. Denial And Ordering Assessment

Denial still validates persisted presentation proof first. It then delegates to
the accepted generic denial path with the fixed registration and an unavailable
source sentinel. The generic denial branch does not invoke the project check or
runtime-fact source and does not write a receipt or report artifact.

Grant ordering remains unchanged: proof, current immutable bundle,
project-validation recheck, source-bound reassessment, approval mutation,
receipt derivation, report construction, selected gates, and artifact
persistence.

## 6. Generic API Compatibility

Existing explicit generic registered-source requests still accept evaluation
time. Those APIs model a lower-level caller-visible source boundary and were not
part of this blocker. No generic API signature or behavior changed.

## 7. Structural And Behavioral Proof

Removing the fields from the public selected request structs is structural
proof that selected callers cannot provide an authority clock. The integration
tests compile only by constructing the new fact-free selected requests.

The focused five-test matrix passes and confirms:

- complete two-gate success through receipt and report-artifact closure;
- denial without decision-time recheck, skill invocation, receipt, or artifact
  write;
- missing presentation proof before recheck or event mutation;
- relevant-definition invalidation before recheck; and
- failed decision-time validation before approval mutation or writes.

The runtime-fact source suite independently retains stale- and future-time
failure coverage for the generic source boundary.

## 8. Privacy And Error Assessment

No timestamp, source identity, path, command output, raw fact, report text,
environment value, provider payload, or credential was added to Debug, errors,
events, receipts, or artifacts. Removing the two Debug fields reduces the
selected public metadata surface. Existing stable error mappings remain
unchanged.

## 9. Validation Reviewed

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Focused selected-consumer local-executor tests: 5 passed.
- `cargo test --workspace`: passed; opt-in live tests remained ignored as
  designed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1786444568117323000-2`.
- Approval ID:
  `approval/run-1786444568117323000-2/review-scope-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/66c8a41e42f1a429`.
- Terminal status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, no retries, and no
  escalations.
- Approval-presentation enforcement: proof enforced with the presentation
  marker present in the durable event trail.

The delegated maintainer performed source review, validation interpretation,
documentation, and git work outside the kernel. The kernel governed scope and
approval and retained the durable phase trail; it did not edit files, execute
checks, mutate git state, push the branch, or merge the pull request.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Keep selected CLI adoption in its own compatibility-sensitive phase.
- Preserve the accepted two-gate behavior when aggregate and workflow step
  approvals are both declared.
- Keep the existing generic source APIs available while the selected consumer
  is adopted and equivalence remains under observation.

## 13. Recommended Next Phase

Merge PR 458. Then plan the explicit selected-consumer CLI adoption phase. Do
not broaden activation, provider execution, SideEffect behavior, schemas,
examples, hosted behavior, or mutation families as part of that planning.
