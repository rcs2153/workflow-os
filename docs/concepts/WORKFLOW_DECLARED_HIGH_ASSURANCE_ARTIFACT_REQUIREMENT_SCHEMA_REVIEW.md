# Workflow-Declared High-Assurance Artifact Requirement Schema Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The workflow-declared high-assurance artifact requirement schema slice stayed within the approved schema/parser/SDK/validation boundary. It adds a small authoring surface for terminal report artifact high-assurance disclosure posture while preserving the core invariant that declared governance must be enforced or rejected. Today, only `not_required` validates; stronger postures are schema-known but fail semantic validation until runtime artifact derivation exists.

The next P0 work should be the governed phase approval handoff instruction fix documented in [Governed Phase Approval Handoff Context Bug](GOVERNED_PHASE_APPROVAL_HANDOFF_CONTEXT_BUG.md), because approval checkpoints must present complete human-review context consistently before more governed build phases proceed. After that, the next feature phase should return to workflow-declared artifact requirement runtime derivation.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented scope:

- Rust workflow definition field for `report_artifact_requirements`;
- JSON schema field and enum vocabulary;
- TypeScript SDK workflow type surface;
- parser coverage for known vocabulary and fail-closed unknown field/value handling;
- semantic validation that accepts `not_required` and rejects stronger postures until runtime enforcement exists;
- TypeScript contract fixture coverage;
- docs and end-of-phase report.

No accidental implementation was found for:

- runtime derivation from workflow declarations;
- automatic report generation;
- automatic report artifact writing;
- executor artifact request construction from workflow declarations;
- CLI artifact behavior;
- example updates;
- workflow-declared high-assurance approval controls;
- RBAC, IdP, quorum approval, or revocation enforcement;
- approval evidence attachment;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 3. Schema Field Assessment

The chosen workflow-level field is appropriately narrow:

```yaml
report_artifact_requirements:
  high_assurance_approval: not_required
```

The placement is good because the requirement applies to terminal report artifacts for the workflow rather than step approval behavior, audit events, or telemetry. It avoids overloading policy rules with artifact persistence semantics.

The field is posture-only and does not store approval payloads, actor IDs, evidence payloads, paths, logs, provider payloads, or secret-like values.

## 4. Rust Parser And Model Assessment

The Rust model adds `ReportArtifactRequirements` with `#[serde(default, deny_unknown_fields)]` and reuses the existing `WorkReportArtifactHighAssuranceRequirement` vocabulary. `WorkflowDefinition` defaults the field, so existing workflow specs remain compatible.

The implementation is minimal and domain-aligned:

- no new duplicate posture enum was created;
- no runtime behavior was introduced through model parsing;
- unknown nested fields fail parsing;
- unknown enum values fail parsing;
- the model remains schema-facing and explicit in comments.

## 5. Validation Semantics Assessment

Semantic validation preserves the important governance boundary:

- absent field defaults to no requirement;
- `high_assurance_approval: not_required` passes;
- `disclosure_required`, `validated_disclosure_required`, and `validated_fail_closed_disclosure_required` fail with `validation.workflow.report_artifact_requirement.runtime_not_enforced`;
- the diagnostic points to `$.report_artifact_requirements.high_assurance_approval`;
- validation does not write artifacts, generate reports, inspect runtime state, call adapters, or mutate workflow state.

This is the right fail-closed behavior. It prevents a maintainer from declaring a high-assurance artifact requirement that the runtime cannot yet enforce.

## 6. JSON Schema Assessment

The checked-in v0 workflow schema includes `report_artifact_requirements` with `additionalProperties: false` and a bounded enum:

- `not_required`;
- `disclosure_required`;
- `validated_disclosure_required`;
- `validated_fail_closed_disclosure_required`.

Keeping future enforcement values schema-known while semantic validation rejects them is acceptable for v0 because it lets the parser/SDK vocabulary stabilize without falsely accepting unenforced governance.

No workflow schema version change was introduced.

## 7. TypeScript SDK And Contract Assessment

The TypeScript SDK exposes:

- `ReportArtifactHighAssuranceApprovalRequirement`;
- `ReportArtifactRequirements`;
- optional `report_artifact_requirements` on workflow input/output types.

The SDK does not add runtime behavior, artifact writes, report generation, or high-assurance approval enforcement. Contract fixtures cover a valid `not_required` workflow and a rejected enforcement posture workflow.

One implementation report wording issue was corrected during this review: the report now says TypeScript SDK source types and build validation were updated, not tracked generated `dist` output.

## 8. Runtime Boundary Assessment

The runtime boundary is clean.

The phase does not alter:

- `LocalExecutor::execute(...)`;
- `LocalExecutor::execute_with_report(...)`;
- report artifact write APIs;
- workflow events;
- audit projection;
- state backend behavior;
- CLI behavior.

The implementation correctly treats workflow declarations as authored intent that must be either enforceable or rejected. Runtime derivation remains deferred.

## 9. Privacy And Redaction Assessment

The schema field is redaction-safe by shape. It stores enum posture only and does not accept raw payloads.

No leakage path was found for:

- raw approval payloads;
- actor IDs;
- evidence payloads;
- policy payloads;
- provider payloads;
- command output;
- CI logs;
- Jira or GitHub bodies;
- raw spec contents;
- parser payloads;
- local paths;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values.

Validation errors use stable codes and do not need to echo untrusted values.

## 10. Test Quality Assessment

Test coverage is focused and adequate for this schema slice:

- parser accepts the field and maps posture to artifact gate policy vocabulary;
- unknown nested field is rejected;
- unknown enum value is rejected;
- semantic validation passes `not_required`;
- semantic validation rejects an enforcement posture with the expected stable code;
- TypeScript contract fixtures cover valid and rejected generated workflow specs;
- broader workspace tests exercise existing WorkReport, validation, executor, artifact, and SDK surfaces.

Non-blocking test follow-ups:

- add a direct JSON-schema validation fixture for unknown nested fields if the repository adds a first-class schema-validator harness;
- reduce repeated inline YAML in validation tests only if a local fixture helper emerges naturally.

## 11. Documentation Review

Docs correctly state that:

- workflow specs may declare the schema-facing field;
- `not_required` is the only currently accepted posture;
- stronger postures are known but rejected until runtime derivation exists;
- runtime report generation is not implemented by this field;
- automatic artifact writing is not implemented by this field;
- executor artifact request construction from workflow declarations is not implemented;
- CLI artifact behavior is not implemented;
- examples were not updated;
- approval evidence attachment, side-effect execution, write-capable adapters, hosted behavior, reasoning lineage, and release posture changes remain unsupported.

The phase report, workflow spec docs, roadmap, and adjacent plans are consistent after the minor `dist` wording correction.

## 12. Blockers

No blockers remain for the workflow-declared high-assurance artifact requirement schema slice.

Separate P0 process blocker:

- The governed phase runner/helper must emit an explicit approval handoff block that agents preserve in user-facing approval requests. This is tracked in [Governed Phase Approval Handoff Context Bug](GOVERNED_PHASE_APPROVAL_HANDOFF_CONTEXT_BUG.md).

## 13. Non-Blocking Follow-Ups

- Implement the governed phase approval handoff instruction fix before more long-running governed build phases.
- Plan runtime derivation from workflow declarations into explicit report artifact gate inputs.
- Keep examples unchanged until runtime derivation is implemented and reviewed.
- Add direct schema-validator fixture coverage for unknown nested fields if schema-only validation becomes a standard contract check.

## 14. Recommended Next Phase

Recommended next phase: governed phase approval handoff instruction fix.

Why: the schema slice is acceptable, but the live dogfood issue showed that approval context can be lost in the agent handoff even when the runner prints the right fields. The kernel should make the approval request payload harder to collapse or omit before more governed phases depend on it.

After that fix, proceed to workflow-declared artifact requirement runtime derivation planning or implementation, still opt-in and local only.

## 15. Validation

Validation commands run:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:contracts` - passed.
- `npm run check:ts` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

## 16. Dogfood Governance

This review phase was governed by the local Workflow OS dogfood runner.

- workflow phase: review
- workflow ID: `dg/review`
- run ID: `run-1783132276191106000-2`
- approval ID: `approval/run-1783132276191106000-2/review-scope-approved`
- approval outcome: approved by the maintainer before review completion
- close status: completed
- event summary: 39 events total; 1 approval; 0 retries; 0 escalations

The dogfood runner coordinates governance only. The review, documentation edits, and validation commands were performed by the executor.
