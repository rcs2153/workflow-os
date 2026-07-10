# Explicit Projected Proof-Marker Artifact Composition Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation composes previously accepted primitives into a narrow, explicit artifact-capable path. It does not change default executor behavior, does not make report artifacts automatic, and does not introduce provider writes, CLI behavior, schemas, examples, hosted runtime behavior, reasoning lineage, or release posture changes.

Recommended next phase: approval-resume artifact/projection composition planning.

## 2. Scope Verification

The phase stayed within the approved explicit composition scope.

Implemented scope:

- explicit `LocalExecutionProjectedProofMarkerArtifactInputs`;
- explicit `LocalExecutionWithProjectedProofMarkerArtifactResult`;
- explicit `execute_with_report_artifact_and_projected_proof_markers(...)`;
- caller-supplied projection store;
- caller-supplied projection persistence policy;
- caller-supplied artifact proof-marker gate policy;
- successful projection plus artifact write tests;
- projection failure plus no artifact write tests;
- roadmap, plan, and phase report updates.

No accidental scope expansion was found:

- no default executor projection persistence;
- no automatic report generation;
- no automatic report artifact writing;
- no CLI artifact behavior;
- no workflow schema changes;
- no examples;
- no runtime config;
- no provider calls;
- no provider writes;
- no side-effect execution;
- no hosted or distributed runtime behavior;
- no reasoning lineage;
- no recursive agents or agent swarms;
- no Level 3/4 autonomy changes;
- no release posture changes.

## 3. API Assessment

The API is additive and appropriately narrow.

`LocalExecutionProjectedProofMarkerArtifactInputs` makes the projection store, projection policy, selected approval references, sensitivity, redaction metadata, and proof-marker artifact gate policy explicit. This is the right boundary because the helper must not discover hidden stores or silently persist projection posture.

`LocalExecutionWithProjectedProofMarkerArtifactResult` preserves the existing `LocalExecutionWithReportArtifactResult` and adds projection persistence posture separately. That keeps the already-reviewed artifact result stable while allowing callers to inspect whether projection persistence succeeded.

The exported helper is correctly adjacent to existing local executor/report APIs and does not alter `LocalExecutor::execute(...)` or the existing report artifact helpers.

## 4. Composition Assessment

The implementation follows the planned composition order:

1. Execute or rehydrate through the existing artifact-capable execution path.
2. Return the original execution error if no run exists.
3. Return terminal-report status posture for non-terminal runs.
4. Persist approval proof-marker projections from the terminal run into the caller-supplied projection store.
5. Stop before report generation and artifact writing when projection persistence fails.
6. Generate the terminal `WorkReport`.
7. Construct the `WorkReportArtifactRecord`.
8. Evaluate existing artifact gates, including the store-backed proof-marker gate.
9. Write the artifact only when requested gates pass.

This closes the hand-populated projection store gap for the explicit artifact path without making projection persistence global behavior.

## 5. Workflow Semantics Assessment

Workflow semantics are preserved.

- Execution failure before a run exists remains an execution `Err`.
- Non-terminal execution returns a run plus report status posture and no projection persistence.
- Projection persistence failure after a run exists returns the run plus projection error.
- Projection persistence failure does not generate a report.
- Projection persistence failure does not write an artifact.
- Report generation and artifact write failures remain post-run posture.
- The helper does not append workflow events.
- The helper does not mutate workflow run history.
- The helper does not execute side effects.
- The helper does not call providers.

The tests assert event-history preservation for both success and projection-failure cases.

## 6. Gate And Policy Assessment

The helper composes two policy layers cleanly:

- projection persistence policy decides what approval decision events should be projected;
- artifact proof-marker gate policy decides what persisted projection posture the artifact must prove.

Workflow-declared proof-marker artifact requirements still flow through the existing `execute_for_report_artifact_path(...)` derivation path, so caller policy cannot weaken authored workflow requirements in the explicit artifact path.

Projection persistence records observed proof posture before the artifact gate evaluates. This is acceptable because projection records are bounded posture records, not artifact writes or provider actions.

## 7. Privacy And Redaction Assessment

The privacy posture is acceptable.

The helper relies on existing constructors and persistence helpers for validation and redaction. It does not copy approval presentation payloads, approval handoff text, approval reasons, command output, provider payloads, raw source/spec contents, environment values, credentials, authorization headers, private keys, token-like values, or secret-like values.

`Debug` for the new input redacts the projection store and redaction metadata. `Debug` for the result exposes only the underlying bounded artifact result posture, whether projection persistence exists, and a projection error code.

The focused success test confirms debug output does not leak approval or presentation identifiers.

## 8. Error-Handling Assessment

Error behavior is stable and non-misleading.

- Projection persistence errors are returned as projection posture, not workflow execution failure.
- Projection persistence errors do not trigger partial artifacts.
- Missing proof-marker projection uses the stable code `approval_proof_marker_projection_persistence.marker_missing`.
- Tests assert projection errors do not leak the approval identifier.
- Artifact write errors remain artifact posture through the existing artifact result.

No misleading user project diagnostic behavior was introduced.

## 9. Test Quality Assessment

The new tests cover the most important behavior:

- successful projection persistence plus artifact write;
- projection persistence result exposure;
- artifact write after proof-marker projection;
- workflow event preservation;
- debug non-leakage for projection input and result;
- projection failure before report generation;
- projection failure before artifact writing;
- no partial projection records on missing proof;
- no artifact records on projection failure;
- stable non-leaking error code for missing marker.

Existing tests continue to cover hand-populated projection stores, proof-marker gate behavior, workflow-declared requirement strengthening, artifact write failure posture, report generation behavior, WorkReport/WorkReportArtifact privacy, approval presentation proof markers, and projection persistence helper behavior.

Non-blocking test follow-up: add a focused non-terminal test for this exact wrapper to assert it returns terminal-report status posture and skips projection persistence. Existing lower-level report/artifact tests cover non-terminal behavior, so this is not a blocker.

## 10. Documentation Review

Documentation is accurate.

The roadmap, implementation plan, and phase report state that explicit projected proof-marker artifact-path composition is implemented.

They also continue to state that the following remain unimplemented:

- default executor enforcement;
- automatic artifact writing;
- automatic projection persistence;
- CLI artifact behavior;
- examples;
- provider writes;
- hosted runtime;
- reasoning lineage;
- release posture changes.

The phase report includes completed scope, non-scope, API summary, composition behavior, workflow semantics, privacy posture, test coverage, commands run, governed implementation record, limitations, and next phase.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add a direct non-terminal wrapper test for `execute_with_report_artifact_and_projected_proof_markers(...)`.
- Consider a future typed convenience policy for the common projection/gate pairing, but only after more runtime use proves the shape.
- Keep approval-resume artifact/projection behavior separate until planned and reviewed.

## 13. Recommended Next Phase

Recommended next phase: approval-resume artifact/projection composition planning.

The current helper covers terminal execution or rehydration through the explicit artifact path. Approval-resume paths remain separate and should not be widened by assumption. A small planning phase should decide whether and how approval-resume APIs should compose projection persistence and artifact gates while preserving the same explicit-store, opt-in, no-default-runtime-change posture.

## 14. Validation

Review validation:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.

## 15. Governed Review Record

- Dogfood workflow: `dg/review`
- Run ID: `run-1783689708309704000-2`
- Approval ID: `approval/run-1783689708309704000-2/review-scope-approved`
- Approval outcome: granted by delegated maintainer
- Approval presentation ID: `presentation/c4b27b78124fe5ee`
- Approval presentation hash: `c4b27b78124fe5ee12089b8195369bdc7e3fb0de693daef292a7ae9b64c50ee3`
- Event summary: 39 events; `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Out-of-kernel work: review writing, validation commands, git/PR actions, and GitHub merge actions remain performed by the delegated maintainer execution layer outside the Workflow OS runtime.
