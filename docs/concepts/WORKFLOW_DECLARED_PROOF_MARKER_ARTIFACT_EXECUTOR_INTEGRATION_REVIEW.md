# Workflow-Declared Proof-Marker Artifact Executor Integration Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The explicit executor artifact-path integration is correctly bounded. Workflow-declared approval proof-marker artifact requirements are accepted only when the caller uses the explicit proof-marker-capable artifact executor path, and the selected workflow declaration is composed with caller policy by strictness before artifact write.

Default executor paths remain conservative, and artifact paths without proof-marker gate inputs still reject enforceable workflow-declared approval proof-marker artifact requirements.

Recommended next phase: approval proof-marker projection persistence planning for executor-adjacent artifact paths.

## 2. Scope Verification

The phase stayed within the approved executor artifact-path integration scope.

Implemented scope:

- explicit `execute_with_report_artifact_and_proof_marker_gates(...)` integration with workflow-declared approval proof-marker artifact requirements;
- strict policy composition between selected workflow declaration and caller-supplied proof-marker artifact gate policy;
- `ProjectValidationCapability::ReportArtifactCapable` split between high-assurance artifact capability and approval proof-marker artifact capability;
- rejection of workflow-declared proof-marker artifact requirements when callers use the artifact path without proof-marker gate inputs;
- focused executor regression tests;
- roadmap, plan, and implementation report updates.

No accidental scope expansion was found:

- no default executor enforcement;
- no automatic report generation;
- no automatic report artifact writing;
- no automatic approval proof-marker projection persistence;
- no approval-resume API change;
- no CLI behavior;
- no schema changes;
- no examples;
- no provider writes;
- no side-effect execution;
- no hosted or distributed runtime behavior;
- no reasoning lineage;
- no release posture changes.

## 3. Executor Integration Assessment

The selected integration point is appropriate: `execute_with_report_artifact_and_proof_marker_gates(...)`.

That path already requires explicit artifact store inputs, side-effect store inputs, proof-marker projection store inputs, and a proof-marker artifact gate policy. It is therefore the smallest runtime surface that can honestly enforce workflow-declared approval proof-marker artifact requirements before artifact write.

The implementation derives artifact policies during the artifact-capable execution preparation path and again during rehydrated-run artifact requests. This preserves selected-workflow identity checks and prevents a stale or mismatched workflow declaration from being applied to the wrong run.

Review found no evidence that `LocalExecutor::execute(...)`, `LocalExecutor::execute_with_report(...)`, approval decision APIs, cancellation APIs, or CLI paths were broadened.

## 4. Validation Boundary Assessment

The validation boundary is materially improved.

`ProjectValidationCapability::ReportArtifactCapable` now distinguishes:

- artifact-capable validation for workflow-declared high-assurance report artifact requirements; and
- proof-marker-capable validation for workflow-declared approval proof-marker artifact requirements.

This avoids false governance. A caller that can write report artifacts is not automatically treated as capable of enforcing approval proof-marker projection requirements.

Default validation still rejects:

```text
report_artifact_requirements.approval_proof_markers: projection_required
report_artifact_requirements.approval_proof_markers: marker_required
```

unless the selected workflow is validated through the explicit proof-marker-capable artifact path.

## 5. Policy Composition Assessment

The policy composition is correct for the current vocabulary.

The effective policy is the stricter of:

- the selected workflow declaration; and
- the caller-supplied proof-marker artifact gate policy.

Review confirmed:

- workflow `projection_required` strengthens a disabled caller policy into projection-required behavior;
- workflow `marker_required` strengthens a marker-free caller policy into present-marker-required behavior;
- caller policy can strengthen a workflow declaration;
- caller policy cannot weaken an authored workflow requirement;
- workflow `not_required` preserves existing caller-supplied proof-marker behavior.

This is the right posture for authored governance: workflow declarations are contracts, not suggestions, once a capable executor path accepts them.

## 6. Failure Semantics Assessment

Failure semantics remain conservative and non-destructive.

When proof-marker artifact validation fails after a run already exists, the result preserves:

- the workflow run;
- the terminal run status;
- the generated in-memory report when available;
- no persisted artifact;
- a stable artifact write error.

The failure does not rewrite workflow pass/fail semantics. It does not append workflow events, mutate run snapshots, create projection records, repair missing proof, or write partial artifacts.

This matches the established report/artifact boundary: report artifact failure is artifact posture, not retroactive workflow execution failure.

## 7. Privacy And Redaction Assessment

The integration preserves the accepted proof-marker privacy boundary.

Review found no copying of:

- approval presentation payloads;
- approval reason text;
- report text beyond already validated model construction;
- provider payloads;
- command output;
- raw spec contents;
- source file contents;
- local paths in errors;
- tokens, credentials, authorization headers, private keys, or secret-like values.

Focused tests assert that missing projection and marker-required artifact errors do not leak workflow-declared approval reference strings. Error codes are stable and bounded.

## 8. Test Quality Assessment

The focused tests cover the critical behavior:

- workflow-declared `projection_required` strengthens a disabled caller proof-marker policy;
- workflow-declared `marker_required` strengthens a marker-free caller proof-marker policy;
- artifact paths without proof-marker gate inputs reject workflow-declared proof-marker artifact requirements;
- existing artifact path behavior remains preserved when proof-marker gate inputs are absent;
- no artifact is written when the proof-marker gate fails;
- run and report are preserved when artifact validation fails.

Existing workspace validation also passed in the implementation phase:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

Non-blocking test follow-up: add an approval-gated workflow scenario once a scoped approval-resume artifact path exists. Today approval-resume APIs intentionally remain outside this phase, so this is not a blocker for the explicit artifact path under review.

## 9. Documentation Review

The roadmap, implementation plan, and implementation report accurately state that:

- explicit executor artifact-path derivation is implemented;
- default executor enforcement is not implemented;
- automatic report generation is not implemented;
- automatic report artifact writing is not implemented;
- automatic approval proof-marker projection persistence is not implemented;
- approval-resume paths are not broadened;
- CLI behavior is not implemented;
- schemas beyond the already implemented vocabulary are not changed;
- examples are not updated;
- provider writes, hosted behavior, reasoning lineage, side-effect execution, and release posture changes remain unimplemented.

No dangerous false claims were found.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Plan how approval-resume or approval-decision paths should eventually participate in workflow-declared proof-marker artifact requirements without changing default approval behavior.
- Plan automatic approval proof-marker projection persistence for executor-adjacent artifact paths, still opt-in and local.
- Consider a short doc note explaining that authored `approval_proof_markers` requirements require the explicit proof-marker artifact path to be enforceable.
- Keep CLI artifact rendering, schema broadening, examples, provider writes, hosted behavior, and release posture changes deferred.

## 12. Recommended Next Phase

Recommended next phase: approval proof-marker projection persistence planning for executor-adjacent artifact paths.

Reason: the executor can now consume workflow-declared proof-marker artifact requirements when a caller supplies a proof-marker projection store and policy, but projection records are still caller-supplied. The next runtime-composition gap is deciding when and how bounded approval proof-marker projection records should be persisted from already proof-enforced approval events, without making artifact writing automatic or changing default approval behavior.

## 13. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783682330508208000-2`.
- Approval ID: `approval/run-1783682330508208000-2/review-scope-approved`.
- Approval presentation ID: `presentation/2254c056c283af50`.
- Approval presentation hash: `2254c056c283af50236334b2fb01d7d3e0f2f70bdfbbc73fa4cd2e694c820f25`.
- Approval outcome: granted by delegated maintainer for review-only scope.

## 14. Validation Commands Run

- `npm run dogfood:benchmark -- phase-start --phase review --work-summary "review workflow-declared proof-marker artifact executor integration" --approved-scope "create phase-level maintainer review for explicit executor artifact-path proof-marker derivation integration" --strict-non-goals "no implementation changes, no default executor changes, no artifact automation, no projection persistence, no CLI behavior, no schemas" --expected-touched-surfaces "docs/concepts review only" --validation-required "npm run check:docs" --why-now "implementation report recommends review before further proof-marker artifact expansion"` - passed.
- `./target/debug/workflow-os --project-dir ./dogfood/workflow-os-self-governance --state-dir /var/folders/r9/y7_mqmq108z94yhyt702h2b80000gn/T/workflow-os-self-governance-state --mock-all-local-skills dogfood approval-presentation approve --run-id run-1783682330508208000-2 --approval-id approval/run-1783682330508208000-2/review-scope-approved --presentation-id presentation/2254c056c283af50 --actor user/delegated-maintainer --reason approved-proof-marker-artifact-executor-integration-review` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783682330508208000-2 --phase review` - passed.

Phase-close summary:

- status: `Completed`;
- events: 39 total;
- approvals: 1;
- retries: 0;
- escalations: 0;
- approval-presentation enforcement: `proof_enforced`;
- approval-presentation event marker: present.
