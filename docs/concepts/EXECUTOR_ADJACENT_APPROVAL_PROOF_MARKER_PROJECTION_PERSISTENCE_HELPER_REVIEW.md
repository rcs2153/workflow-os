# Executor-Adjacent Approval Proof-Marker Projection Persistence Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The helper is appropriately narrow, explicit, local, and executor-adjacent. It persists bounded approval proof-marker projection records from supplied `WorkflowRun` approval decision events into a caller-supplied `LocalApprovalProofMarkerAuditProjectionStore`, without changing default executor behavior or making projection persistence automatic.

Recommended next phase: explicit artifact-path composition planning.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented scope:

- `persist_approval_proof_marker_projections_for_run(...)`;
- explicit input, policy, result, record-result, and disposition types;
- event-derived projection persistence from approval decision events only;
- support for proof-marked granted decisions;
- support for proof-marked denied decisions under default policy;
- marker-free skip behavior by default;
- fail-closed behavior for selected marker-free approvals when required;
- duplicate matching-record detection;
- duplicate conflicting-record rejection;
- writes only through `LocalApprovalProofMarkerAuditProjectionStore`;
- redaction-safe `Debug` for new helper inputs/results;
- focused tests and documentation.

No accidental implementation was found for:

- default executor projection persistence;
- automatic projection persistence for all approvals;
- automatic report artifact writing;
- report generation changes;
- CLI behavior;
- workflow schema fields;
- examples;
- provider calls;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The helper API is appropriately explicit and reviewable.

`ApprovalProofMarkerProjectionPersistenceInput` requires callers to provide:

- a `WorkflowRun`;
- a `LocalApprovalProofMarkerAuditProjectionStore`;
- an `ApprovalProofMarkerProjectionPersistencePolicy`;
- optional selected `ApprovalReferenceId` values;
- sensitivity and redaction metadata.

This keeps store selection, projection policy, and invocation timing outside default executor paths. The helper returns bounded counts and record dispositions rather than mutating workflow state or attempting to repair missing approval proof.

The new public exports are consistent with existing `workflow-core` helper surfaces and do not introduce a new runtime execution path.

## 4. Source-Of-Truth Assessment

The source-of-truth boundary is correct.

The helper delegates to `derive_approval_proof_marker_audit_projection(...)`, then persists only records derived from approval decision workflow events. It does not treat approval-presentation records alone, approval reasons, approval handoff text, report prose, CLI output, provider payloads, inferred approval IDs, or fabricated proof markers as proof.

This preserves the core distinction:

```text
Approval presentation proves context existed; approval decision proof marker proves the decision used that context.
```

## 5. Persistence Behavior Assessment

Persistence behavior matches the plan.

Verified behavior:

- proof-marked granted approvals persist one projection record;
- proof-marked denied approvals persist under default policy;
- denied approvals can be excluded through policy;
- marker-free approval decisions are skipped by default;
- marker-free selected approvals fail closed when `require_selected_approvals_projected` is enabled;
- no selected matching approval decisions returns a stable no-approval-events error;
- matching duplicate durable records are reported as already present;
- conflicting duplicate durable records fail closed;
- store read/write/list failures are mapped to stable non-leaking errors.

The local projection store uses duplicate-safe create semantics and deterministic listing. The helper pre-reconciles existing records before write, which makes matching duplicates idempotent and conflicting duplicates fail closed.

## 6. Workflow Semantics Assessment

Workflow semantics are preserved.

The helper does not:

- execute workflows;
- approve requests;
- append workflow events;
- mutate `WorkflowRun`;
- mutate `WorkflowRunSnapshot`;
- mutate approval state;
- mutate report state;
- write report artifacts;
- create side-effect records;
- call providers;
- discover hidden stores;
- change workflow pass/fail status.

Projection persistence failure remains a structured helper error. It does not retroactively fail, complete, resume, cancel, or otherwise change the run.

## 7. Error-Handling Assessment

Error handling is conservative and non-misleading.

Stable error codes include:

- `approval_proof_marker_projection_persistence.no_approval_events`;
- `approval_proof_marker_projection_persistence.marker_missing`;
- `approval_proof_marker_projection_persistence.duplicate_conflict`;
- `approval_proof_marker_projection_persistence.invalid_projection_record`;
- `approval_proof_marker_projection_persistence.store_write_failed`.

The helper intentionally maps lower-level redaction or projection-construction failures to bounded persistence errors. Review found no error paths that include approval IDs, presentation IDs, run IDs, store paths, approval reasons, handoff text, report text, raw provider payloads, command output, tokens, credentials, or secret-like values.

## 8. Privacy And Redaction Assessment

The privacy posture is acceptable for this phase.

Persisted records contain bounded projection posture:

- projection record reference;
- source workflow event reference;
- approval reference;
- workflow and run identity;
- decision kind;
- proof-marker present posture;
- booleans for presentation ID/content hash presence;
- sensitivity and validated redaction metadata.

The helper does not persist or copy:

- approval-presentation payloads;
- approval handoff text;
- approval reasons;
- report text;
- command output;
- provider payloads;
- raw source or spec contents;
- environment values;
- credentials;
- tokens;
- authorization headers;
- private keys;
- secret-like values.

`Debug` output for the new input, record result, and aggregate result is redaction-safe.

## 9. Test Quality Assessment

Focused tests are sufficient for acceptance.

Tests cover:

- granted proof-marker projection persistence;
- denied proof-marker projection persistence;
- marker-free skip behavior;
- marker-free selected approval fail-closed behavior;
- matching duplicate record as already present;
- conflicting duplicate rejection;
- event-history preservation;
- persisted identity, source event, approval reference, decision, marker posture, sensitivity, and redaction posture;
- debug non-leakage for approval, presentation, and local test identity strings.

Existing workspace tests cover adjacent WorkReport, WorkReportContract, EvidenceReference, Diagnostic, validation, adapter telemetry, runtime, side-effect, artifact, approval-linkage, and proof-marker gate behavior.

Non-blocking test follow-ups:

- add a focused test for `granted_only()` skipping denied proof-marked approvals;
- add a focused test for selected approval IDs that do not appear in the run, if future callers need a more specific distinction than `no_approval_events`;
- add a composition-level test once an artifact-capable executor path calls this helper before proof-marker-gated artifact writes.

## 10. Documentation Review

Documentation is honest and scoped.

Verified docs say:

- executor-adjacent approval proof-marker projection persistence is implemented;
- the helper is explicit, local, and caller-supplied-store bounded;
- default executor behavior is unchanged;
- automatic projection persistence is not implemented;
- automatic report artifact writing is not implemented;
- CLI behavior is not implemented;
- workflow schema fields are not added;
- examples are not updated;
- provider writes, side-effect execution, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

No dangerous false claims were found.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add focused `granted_only()` denied-decision skip coverage.
- Consider whether missing selected approval IDs should eventually return a more specific error than `no_approval_events`.
- Plan explicit artifact-path composition that persists proof-marker projections before validating proof-marker artifact gates.
- Keep default executor behavior, automatic projection persistence, automatic artifact writing, CLI behavior, schemas, examples, provider writes, hosted behavior, reasoning lineage, and release posture changes deferred.

## 13. Recommended Next Phase

Recommended next phase: explicit artifact-path composition planning.

Reason: the helper is now accepted as the durable projection persistence building block. The next runtime-composition question is how an explicit artifact-capable caller should compose terminal execution, proof-marker projection persistence, terminal report construction, proof-marker gate validation, and artifact writing without making any of those behaviors automatic or changing default workflow semantics.

## 14. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783686263376356000-2`.
- Approval ID: `approval/run-1783686263376356000-2/review-scope-approved`.
- Approval presentation ID: `presentation/48051d7072944514`.
- Approval presentation hash: `48051d7072944514d45d2c596d83e617665c66c76987e1d0204fd60e1bd45d4e`.
- Approval outcome: granted by delegated maintainer for review-only scope.
- Approved scope: create the maintainer review report; inspect code, tests, plan, report, and docs; run validation.
- Strict non-goals: no implementation changes, default persistence, artifact writes, CLI, schemas, examples, providers, writes, hosted behavior, lineage, or release changes.

## 15. Validation Commands Run

- `npm run dogfood:benchmark -- phase-start --phase review ...` - passed.
- `./target/debug/workflow-os --project-dir ./dogfood/workflow-os-self-governance --state-dir /var/folders/r9/y7_mqmq108z94yhyt702h2b80000gn/T/workflow-os-self-governance-state --mock-all-local-skills dogfood approval-presentation approve --run-id run-1783686263376356000-2 --approval-id approval/run-1783686263376356000-2/review-scope-approved --presentation-id presentation/48051d7072944514 --actor user/delegated-maintainer --reason approved-review-phase-scope` - passed.
- `cargo fmt --all --check` - passed using the repository bundled Rust toolchain.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed using the repository bundled Rust toolchain.
- `cargo test --workspace` - passed using the repository bundled Rust toolchain.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783686263376356000-2 --phase review` - passed.

Phase-close summary:

- status: `Completed`;
- events: 39 total;
- approvals: 1;
- retries: 0;
- escalations: 0;
- approval-presentation enforcement: `proof_enforced`;
- approval-presentation event marker: present.
