# Workflow-Declared Proof-Marker Artifact Executor Integration Report

## 1. Executive Summary

This phase implemented the explicit executor artifact-path integration for workflow-declared approval proof-marker artifact requirements.

The executor now derives an effective approval proof-marker artifact gate policy from the selected workflow declaration and the caller-supplied proof-marker gate policy when, and only when, the explicit `execute_with_report_artifact_and_proof_marker_gates(...)` path is used.

Default executor paths remain conservative. `projection_required` and `marker_required` still fail semantic validation outside the explicit proof-marker artifact path.

## 2. Scope Completed

- Wired workflow-declared approval proof-marker artifact requirement derivation into the explicit proof-marker artifact executor path.
- Composed workflow-declared policy with caller-supplied proof-marker gate policy by strictness.
- Preserved caller-supplied proof-marker behavior when the workflow declares `not_required`.
- Split artifact-capable validation so high-assurance artifact capability and approval proof-marker artifact capability are not treated as the same enforcement posture.
- Ensured artifact paths without proof-marker gate inputs still reject enforceable workflow proof-marker requirements.
- Added focused executor regression tests.
- Updated roadmap and planning documentation.

## 3. Scope Explicitly Not Completed

- No default executor enforcement.
- No automatic report artifact writing.
- No automatic approval proof-marker projection persistence.
- No approval-resume capability changes.
- No CLI behavior.
- No schema changes.
- No examples.
- No provider writes.
- No side-effect execution.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Executor Behavior Summary

`execute_with_report_artifact_and_proof_marker_gates(...)` now:

- loads the selected workflow with report-artifact capability and proof-marker capability enabled;
- derives high-assurance artifact policy as before;
- derives approval proof-marker artifact policy from the selected workflow and caller policy;
- uses the effective proof-marker policy before artifact write;
- preserves the run and in-memory report when artifact proof-marker validation fails after a terminal run exists.

`execute_with_report_artifact_and_side_effect_gates(...)` remains unable to accept workflow-declared approval proof-marker artifact requirements because it does not receive a projection store or proof-marker gate policy.

## 5. Validation Boundary Summary

`ProjectValidationCapability::ReportArtifactCapable` now carries an explicit `approval_proof_marker_capable` flag.

High-assurance artifact requirements may be accepted for the selected workflow by the explicit artifact path. Approval proof-marker artifact requirements are accepted only when that artifact path also has proof-marker gate capability.

This prevents false governance where a workflow declaration would validate without an executor path capable of enforcing the declared proof-marker gate.

## 6. Policy Composition Summary

The effective proof-marker artifact policy is the strictest policy implied by:

- the selected workflow declaration; and
- the caller-supplied proof-marker gate policy.

This means:

- `projection_required` can strengthen a disabled caller policy into projection-required behavior;
- `marker_required` can strengthen marker-free caller policy into marker-present behavior;
- callers cannot weaken authored workflow requirements.

## 7. Failure Semantics

If proof-marker artifact validation fails after the run already exists, the executor returns:

- the completed run;
- the generated in-memory report when available;
- no report artifact;
- a stable, non-leaking artifact write error.

Workflow pass/fail semantics are not rewritten by artifact validation failure.

## 8. Privacy And Redaction Summary

The implementation does not copy raw approval payloads, raw report payloads, provider output, command output, tokens, paths, or source contents into errors.

Tests assert non-leakage for approval references and workflow-declared test markers in artifact validation errors.

## 9. Test Coverage Summary

Added focused tests for:

- workflow-declared `projection_required` strengthening a disabled caller proof-marker policy;
- workflow-declared `marker_required` strengthening a marker-free caller proof-marker policy;
- rejection of workflow-declared proof-marker artifact requirements when no proof-marker gate path is selected;
- existing caller-supplied proof-marker gate behavior.

Existing proof-marker artifact tests continue to cover successful persisted projection writes, missing projection behavior, and unchanged behavior when no proof-marker gate is requested.

## 10. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start --phase implementation --work-summary "implement executor artifact path proof-marker derivation integration" --approved-scope "wire reviewed derivation helper into explicit proof-marker artifact executor path" --strict-non-goals "no default executor changes, no automatic artifact writes, no projection persistence, no CLI behavior, no schemas" --expected-touched-surfaces "crates/workflow-core executor validation tests docs" --validation-required "cargo fmt, clippy, cargo test workspace, docs check" --why-now "planning accepted explicit executor artifact path integration as next runtime composition step"` - passed.
- `./target/debug/workflow-os --project-dir ./dogfood/workflow-os-self-governance --state-dir /var/folders/r9/y7_mqmq108z94yhyt702h2b80000gn/T/workflow-os-self-governance-state --mock-all-local-skills dogfood approval-presentation approve --run-id run-1783679642971878000-2 --approval-id approval/run-1783679642971878000-2/implementation-approved --presentation-id presentation/e2a22c25cf09aead --actor user/delegated-maintainer --reason approved-proof-marker-executor-integration-implementation` - passed.
- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test local_executor proof_marker -- --nocapture` - passed.
- `cargo test -p workflow-core --test local_executor report_artifact -- --nocapture` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed after removing one trailing whitespace line from the updated plan.
- `npm run dogfood:benchmark -- phase-close run-1783679642971878000-2 --phase implementation` - passed.

## 11. Remaining Known Limitations

- Approval-resume paths are not broadened by this phase.
- Automatic approval proof-marker projection persistence remains separate.
- Explicit missing-citation records remain deferred.
- Default executor behavior remains intentionally conservative.
- CLI artifact surfaces remain unimplemented.

## 12. Recommended Next Phase

Recommended next phase: workflow-declared proof-marker artifact executor integration review.

This is a safety-sensitive runtime composition boundary. It should be reviewed before any additional expansion into automatic projection persistence, approval-resume capability, CLI artifact surfaces, or write-capable adapter readiness.

## 13. Governed Implementation Record

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1783679642971878000-2`.
- Approval ID: `approval/run-1783679642971878000-2/implementation-approved`.
- Approval presentation ID: `presentation/e2a22c25cf09aead`.
- Approval presentation hash: `e2a22c25cf09aead1ec2d069a75d86bdcbae6c52c07e1bcc5e65961d261b5668`.
- Approval outcome: granted by delegated maintainer for bounded implementation scope.
- Phase-close status: completed.
- Phase-close event summary: 39 events, including one approval request, one approval grant, eight policy decisions, six step schedules, six skill invocation requests, six skill invocation starts, six skill invocation successes, one run resume, and one run completion.
