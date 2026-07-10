# Executor Artifact Proof-Marker Gate Integration Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds a narrow, explicit executor-adjacent artifact path that can require store-backed approval proof-marker projection validation before writing a `WorkReportArtifactRecord`. It preserves default executor behavior, keeps artifact writing opt-in, and does not introduce automatic projection persistence, workflow schema changes, provider writes, CLI behavior, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- `LocalExecutionReportArtifactProofMarkerGateInputs`.
- `execute_with_report_artifact_and_proof_marker_gates(...)`.
- internal sharing between the existing artifact-capable executor path and the new proof-marker-gated path.
- provider-candidate validation preservation for the proof-marker-gated artifact write helper.
- focused executor tests.
- roadmap, implementation-plan, and phase-report updates.

No accidental implementation was found for:

- default executor proof-marker enforcement;
- automatic report artifact writing;
- automatic approval proof-marker projection persistence;
- workflow-declared proof-marker artifact requirements;
- CLI rendering or commands;
- schemas;
- examples;
- provider writes;
- runtime side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- write-capable adapters;
- release posture changes.

## 3. API Assessment

The API is appropriately explicit and additive.

`execute_with_report_artifact_and_proof_marker_gates(...)` requires callers to supply:

- a local executor;
- an artifact store;
- a side-effect record store;
- `LocalExecutionReportArtifactProofMarkerGateInputs`;
- the existing `LocalExecutionWithReportArtifactRequest`.

`LocalExecutionReportArtifactProofMarkerGateInputs` borrows a caller-supplied `LocalApprovalProofMarkerAuditProjectionStore` and carries an explicit `WorkReportArtifactApprovalProofMarkerGatePolicy`. This avoids hidden state discovery and keeps projection persistence outside the executor.

The existing `execute_with_report_artifact_and_side_effect_gates(...)` behavior is preserved through a shared internal helper with no proof-marker gate inputs.

## 4. Gate Composition Assessment

The implemented gate order is appropriate for an opt-in executor artifact path:

1. execute through the existing local executor path;
2. require a terminal run before report generation;
3. generate the terminal report;
4. construct a `WorkReportArtifactRecord`;
5. preserve provider-candidate validation when provider integration inputs are supplied;
6. validate side-effect citation integrity;
7. validate approval-side-effect linkage when side-effect citations require it;
8. validate high-assurance approval disclosure posture;
9. validate store-backed approval proof-marker projections;
10. write the artifact only after all requested gates pass.

The implementation correctly reuses `write_work_report_artifact_with_governance_gates(...)` for the proof-marker path instead of duplicating proof-marker logic in the executor.

## 5. Workflow Semantics Assessment

Workflow semantics are preserved.

Verified:

- execution failure before a run exists still returns `Err`;
- non-terminal runs do not produce artifacts;
- report-generation failure after a run exists remains in the result;
- artifact write failure remains in the result;
- proof-marker gate failure does not change workflow run status;
- no workflow events are appended by the report artifact path;
- no audit or observability events are emitted;
- no projection records are created;
- no side-effect records are created or repaired;
- strict proof-marker failure writes no artifact.

The result shape remains `LocalExecutionWithReportArtifactResult`, which avoids introducing a new public result surface for this narrow integration.

## 6. Error Handling Assessment

The error handling remains fail-closed and non-misleading.

Proof-marker gate failures are reported as artifact write errors after a run and report exist. They do not retroactively fail the workflow run. Missing projection failure is covered through an executor-level test and returns the stable code `work_report_artifact.approval_proof_marker_gate.missing_projection`.

Provider-candidate failures continue to map through existing stable provider artifact write error helpers. Side-effect integrity, approval-linkage, high-assurance disclosure, and artifact-store failures remain delegated to the existing validated helpers.

## 7. Privacy And Redaction Assessment

The privacy posture is acceptable for the phase.

Verified:

- `Debug` for `LocalExecutionReportArtifactProofMarkerGateInputs` redacts the projection store.
- `LocalExecutionWithReportArtifactResult` remains bounded and does not expose report text or artifact payloads in `Debug`.
- tests assert proof-marker presentation fragments and approval IDs do not leak through debug/error strings.
- the executor does not copy approval presentation text, approval reasons, projection payloads, report text, source contents, spec contents, command output, provider payloads, local paths, credentials, tokens, authorization headers, private keys, or secret-like values.

The lower-level proof-marker artifact write input now carries provider integration context and its `Debug` remains bounded through existing provider integration redaction.

## 8. Test Quality Assessment

Test quality is sufficient for acceptance.

Added tests cover:

- artifact write succeeds when a matching persisted approval proof-marker projection exists;
- strict missing projection fails before artifact write;
- proof-marker gate failure preserves run/report posture and event history;
- artifact-store emptiness on proof-marker failure;
- existing artifact-capable executor behavior remains unchanged when proof-marker inputs are absent;
- generated reports cite approval references without recreating `EvidenceReference` values;
- debug/error output does not leak proof-marker presentation fragments or approval identifiers.

Existing tests continue covering:

- default `execute(...)` and `execute_with_report(...)` behavior;
- report artifact side-effect integrity;
- approval-side-effect linkage;
- high-assurance disclosure gates;
- provider-candidate artifact validation;
- proof-marker projection stores;
- WorkReport and WorkReportContract behavior;
- adapter, diagnostic, validation, runtime, and side-effect behavior.

Non-blocking test gaps:

- add one executor-level test combining provider-candidate inputs with proof-marker gate inputs, even though the lower-level provider and proof-marker gates are covered separately;
- add a direct executor-level marker-free projection failure test, mirroring lower-level coverage;
- add a side-effect citation path through the new executor helper to prove approval-linkage still runs with proof-marker inputs.

## 9. Documentation Review

Documentation is honest and scoped.

Verified docs say:

- executor artifact path proof-marker gate integration is implemented;
- the path is explicit and opt-in;
- default executor behavior is not changed;
- automatic artifact writing is not implemented;
- automatic approval proof-marker projection persistence is not implemented;
- workflow-declared proof-marker artifact requirements are not implemented;
- CLI rendering, schemas, examples, provider writes, runtime side-effect execution, hosted behavior, reasoning lineage, write-capable adapters, and release posture changes remain unimplemented.

The phase report records the commands run, known limitations, and the recommended review phase.

## 10. Validation Assessment

Implementation validation recorded in the phase report:

- `cargo fmt --all` passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_artifact_proof_marker -- --nocapture` passed.
- `cargo test -p workflow-core --test local_executor execute_with_report_artifact -- --nocapture` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

Review validation reran the standard checks after this review document was added.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add an executor-level provider-candidate plus proof-marker gate test.
- Add an executor-level marker-free projection strict-policy failure test.
- Add an executor-level side-effect citation and approval-linkage test through the proof-marker helper.
- Plan workflow-declared proof-marker artifact requirements before any default enforcement.

## 13. Recommended Next Phase

Recommended next phase: workflow-declared proof-marker artifact requirements planning.

The opt-in executor path is now accepted. The next design question is whether workflow specs should be able to declare proof-marker artifact requirements and how that declaration should compose with existing artifact requirement validation without making artifact writing automatic, creating hidden projection persistence, adding CLI behavior, or changing default executor semantics.
