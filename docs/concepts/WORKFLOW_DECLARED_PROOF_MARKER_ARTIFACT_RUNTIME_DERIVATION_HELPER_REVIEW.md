# Workflow-Declared Proof-Marker Artifact Runtime Derivation Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The pure workflow-declared approval proof-marker artifact runtime derivation helper is appropriately small, deterministic, and bounded. It composes workflow-declared artifact proof-marker requirements with caller-supplied policy by strictness, fails closed outside artifact-capable derivation mode, and does not add executor integration, artifact writes, projection persistence, CLI behavior, provider writes, hosted behavior, reasoning lineage, or release posture changes.

Recommended next phase: executor artifact-path proof-marker derivation integration planning.

## 2. Scope Verification

The phase stayed within the approved pure-helper scope.

Implemented scope:

- `WorkflowReportArtifactProofMarkerDerivationMode`;
- `WorkflowReportArtifactProofMarkerGateDerivationInput`;
- `WorkflowReportArtifactProofMarkerGateDerivation`;
- `derive_workflow_report_artifact_approval_proof_marker_gate_policy`;
- strictness composition between workflow-declared and caller-supplied proof-marker policies;
- focused tests;
- documentation and phase report updates.

No accidental scope expansion was found:

- no executor artifact-path integration for this derivation helper;
- no automatic report artifact writing;
- no approval proof-marker projection persistence;
- no automatic report generation;
- no default executor proof-marker enforcement;
- no artifact-capable project validation;
- no CLI rendering or artifact commands;
- no examples;
- no provider writes;
- no side-effect execution;
- no hosted or distributed runtime behavior;
- no reasoning lineage;
- no release posture changes.

## 3. Helper API Assessment

The helper API is domain-appropriate and minimal.

The input model requires:

- a selected `WorkflowDefinition`;
- an optional caller-supplied `WorkReportArtifactApprovalProofMarkerGatePolicy`;
- an explicit derivation mode.

The output model exposes only the effective optional approval proof-marker artifact gate policy.

This shape is compatible with future executor artifact-path integration because the caller must explicitly declare whether it is an artifact-capable path. It also keeps derivation separate from validation, report generation, artifact writes, store access, event emission, and executor behavior.

## 4. Composition Assessment

The strictness composition is correct for the current vocabulary:

- `not_required` plus no caller policy yields no proof-marker artifact gate;
- `not_required` plus a stricter caller policy preserves the caller policy;
- `projection_required` strengthens absent or disabled caller policy to projection coverage while allowing marker-free approvals;
- `marker_required` strengthens absent or marker-free caller policy to present-marker coverage;
- caller policy can strengthen but cannot weaken workflow-declared requirements.

The rank mapping is intentionally small and bounded. A caller-supplied policy that does not require projection coverage is treated as disabled, which is the conservative posture for this gate.

## 5. Artifact-Capable Boundary Assessment

The helper fails closed when an enforceable workflow declaration is derived in `DefaultValidation` mode.

Stable error code:

```text
work_report_artifact.approval_proof_marker.derivation.runtime_not_artifact_capable
```

This preserves the current schema behavior: enforceable workflow-declared proof-marker artifact requirements must not silently become accepted in default validation or default executor paths until an explicit artifact-capable path derives and enforces them.

## 6. Validation And Semantics Assessment

The helper is pure and local. Review found no mutation, persistence, executor-state change, artifact write, projection write, event append, provider access, or command execution.

Default executor behavior remains unchanged. Existing semantic validation still rejects enforceable workflow-declared approval proof-marker artifact requirements unless a future artifact-capable runtime path is explicitly designed to derive and enforce them.

## 7. Redaction And Privacy Assessment

The helper operates on bounded enum posture and gate policy only.

Review found no handling or copying of:

- approval presentation payloads;
- approval reasons;
- approval IDs;
- event IDs;
- projection IDs;
- presentation IDs;
- content hashes;
- report text;
- local paths;
- provider payloads;
- command output;
- source or spec contents;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like metadata.

The default-mode error uses a stable code and bounded message. Tests verify the error debug output does not include workflow posture strings or workflow IDs.

## 8. Test Quality Assessment

The focused tests cover:

- `not_required` preserving disabled policy;
- caller-stricter policy preservation;
- workflow `projection_required` strengthening weaker caller policy;
- workflow `marker_required` strengthening weaker caller policy;
- caller strengthening of a workflow projection requirement;
- default-mode rejection for enforceable workflow posture;
- stable non-leaking error code;
- bounded debug output;
- non-mutation of the selected workflow in the no-op path.

Existing workspace validation also passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

Non-blocking test follow-up: before or during executor artifact-path integration, add a table-driven non-mutation assertion across enforceable derivation paths as well as the no-op path. The current implementation takes an immutable reference and is materially safe, but the broader assertion would better document the invariant.

## 9. Documentation Review

The roadmap, runtime derivation plan, and implementation report accurately state that:

- the pure runtime derivation helper is implemented;
- executor artifact-path integration for this helper is not implemented;
- automatic artifact writing is not implemented;
- approval proof-marker projection persistence is not implemented;
- default executor proof-marker enforcement is not implemented;
- artifact-capable project validation is not implemented;
- CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

No dangerous false claims were found.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add broader table-driven non-mutation coverage across enforceable derivation cases before or during executor artifact-path integration.
- Keep the next executor integration phase opt-in and artifact-capable only.
- Ensure future executor integration composes this helper with caller policy before any artifact write and preserves fail-closed behavior for unsupported/default paths.

## 12. Recommended Next Phase

Recommended next phase: executor artifact-path proof-marker derivation integration planning.

Reason: the helper is accepted, but it is intentionally not wired into the artifact-capable executor path yet. Planning should define the smallest explicit integration point, failure behavior, required tests, and non-goals before runtime code consumes workflow-declared proof-marker artifact requirements.

## 13. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783677916248108000-2`.
- Approval ID: `approval/run-1783677916248108000-2/review-scope-approved`.
- Approval presentation ID: `presentation/04115b9fc19af200`.
- Approval presentation hash: `04115b9fc19af200a62453f9acc0322545a8655b8572e4ab3f97276ca9471b65`.
- Approval outcome: granted by delegated maintainer for review-only scope.

## 14. Validation Commands Run

- `npm run dogfood:benchmark -- phase-start --phase review --work-summary "review proof-marker artifact runtime derivation helper" --approved-scope "create phase review document only" --strict-non-goals "no implementation fixes, no executor integration, no artifact writes, no CLI behavior" --expected-touched-surfaces "docs/concepts review report" --validation-required "cargo fmt check, clippy, tests, docs check" --why-now "required review before executor artifact path derivation uses helper"` - passed.
- `./target/debug/workflow-os --project-dir ./dogfood/workflow-os-self-governance --state-dir /var/folders/r9/y7_mqmq108z94yhyt702h2b80000gn/T/workflow-os-self-governance-state --mock-all-local-skills dogfood approval-presentation approve --run-id run-1783677916248108000-2 --approval-id approval/run-1783677916248108000-2/review-scope-approved --presentation-id presentation/04115b9fc19af200 --actor user/delegated-maintainer --reason approved-proof-marker-runtime-derivation-helper-review` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783677916248108000-2 --phase review` - passed.
