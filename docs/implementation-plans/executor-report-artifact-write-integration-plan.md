# Executor Report Artifact Write Integration Plan

Status: Implemented in [Executor Report Artifact Write Integration Report](../concepts/EXECUTOR_REPORT_ARTIFACT_WRITE_INTEGRATION_REPORT.md). This plan did not implement provider writes or automatic artifact behavior.

## 1. Executive Summary

Workflow OS now has:

- an explicit artifact-capable executor path: `execute_with_report_artifact_and_side_effect_gates(...)`;
- workflow-declared high-assurance artifact gate derivation for that explicit path;
- a generic explicit report artifact write integration helper: `write_report_artifact_with_explicit_integrations(...)`;
- an optional GitHub PR comment provider-candidate validation branch inside that helper.

The next question is where the accepted generic helper should be invoked. The recommended answer is narrow: refactor the existing explicit artifact-capable executor path to call the generic helper, while preserving current result shape and current workflow semantics.

This plan does not implement provider writes, live GitHub comments, automatic artifact writing, default executor artifact behavior, CLI mutation behavior, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Goals

- Route the existing explicit artifact-capable executor path through the generic integration helper.
- Preserve workflow-declared high-assurance artifact gate derivation and strictness composition.
- Preserve current `LocalExecutionWithReportArtifactResult` behavior.
- Keep artifact writing explicit and caller-supplied.
- Support a future caller-supplied provider-candidate integration option without making it automatic.
- Keep provider writes out of scope.
- Keep failures structured, bounded, and non-leaking.
- Avoid hidden runtime state and hidden runtime config.

## 3. Non-Goals

Do not implement:

- provider mutation;
- live GitHub PR comment creation;
- runtime side-effect execution;
- automatic artifact writing from default executor paths;
- automatic report generation for every run;
- CLI mutation behavior;
- workflow schema changes;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current State

Already implemented:

- `execute_with_report_artifact_and_side_effect_gates(...)`;
- `LocalExecutionWithReportArtifactRequest`;
- `LocalExecutionReportArtifactInputs`;
- `LocalExecutionWithReportArtifactResult`;
- workflow-declared high-assurance artifact requirement derivation;
- strictness composition between caller policy and workflow-derived policy;
- generic artifact write with `SideEffect` integrity and approval-linkage gates;
- generic explicit integration helper with optional GitHub PR comment provider-candidate validation.

The current executor artifact path still calls:

```rust
write_work_report_artifact_with_side_effect_integrity_and_approval_linkage(...)
```

directly. That is correct but now duplicates the generic integration boundary rather than using it.

## 5. Problem Statement

Workflow OS has two adjacent artifact write boundaries:

1. the executor artifact path that derives report artifacts after a local run;
2. the generic integration helper that composes artifact gates and optional provider-candidate gates.

Leaving them separate too long creates drift risk:

- future provider-candidate validation may be added to the helper but not surfaced to the explicit executor path;
- tests may prove helper behavior without proving the executor-adjacent path;
- workflow-derived high-assurance strictness may remain in the executor path but be invisible to generic helper callers.

The next implementation should align the explicit executor artifact path with the helper without widening the public behavior surface.

## 6. Recommended Implementation Boundary

Modify only the explicit artifact-capable executor path:

```rust
execute_with_report_artifact_and_side_effect_gates(...)
```

Do not change:

- `LocalExecutor::execute(...)`;
- `LocalExecutor::execute_with_report(...)`;
- `execute_with_report_and_side_effect_discovery(...)`;
- CLI commands;
- examples;
- default validation behavior.

The executor artifact path should continue to run only when a caller explicitly invokes the artifact-capable API and supplies artifact inputs/stores.

## 7. API Shape Recommendation

First implementation should be minimal:

- keep `LocalExecutionReportArtifactInputs` fields as-is;
- call `write_report_artifact_with_explicit_integrations(...)` with `ReportArtifactWriteProviderIntegration::None`;
- map the returned `ReportArtifactWriteIntegrationResult` back into the existing `LocalExecutionWithReportArtifactResult`.

This keeps behavior unchanged while centralizing the artifact write boundary.

Provider-candidate executor input should be planned but not required in the first refactor. If added in a later phase, prefer an explicit optional field such as:

```rust
pub provider_integration: Option<LocalExecutionReportArtifactProviderIntegrationInputs>
```

The first variant may be GitHub PR comment validation with:

- expected `SideEffectId`;
- optional workflow events;
- citation policy.

That later option must still not call GitHub or create PR comments.

## 8. Policy Composition

The executor path must preserve existing policy strictness:

- derive workflow-declared high-assurance artifact gate policy;
- compose it with caller-supplied artifact policy by strictness;
- pass the effective policy into `ReportArtifactWriteIntegrationInput`;
- ensure a disabled caller policy cannot weaken workflow-derived requirements.

The generic helper should receive the already-composed effective policy in the first implementation. A later model can introduce a named effective policy wrapper if this becomes ambiguous.

## 9. Provider-Candidate Posture

Provider-candidate integration must remain explicit and supplied by the caller.

For the GitHub PR comment lane, future executor-adjacent integration may pass:

- expected proposed GitHub PR comment `SideEffectId`;
- workflow events for accepted proposed-event proof;
- citation policy.

The path must not:

- infer IDs;
- fabricate citations;
- create `EvidenceReference` values;
- call GitHub;
- write comments;
- copy provider payloads;
- use raw PR bodies or comment bodies.

## 10. Workflow Semantics

The refactor must preserve current workflow semantics:

- execution failure before a run exists still returns `Err`;
- report-generation failure after a run exists remains inside `LocalExecutionWithReportArtifactResult`;
- artifact-write failure after a report exists remains inside `LocalExecutionWithReportArtifactResult`;
- workflow run status is not changed by artifact write success or failure;
- no workflow events are appended;
- no audit or observability events are emitted;
- no side effects are executed.

## 11. Failure Behavior

Failures must remain bounded:

- invalid artifact/run identity;
- missing required side-effect citation;
- missing required side-effect record;
- failed approval linkage;
- missing high-assurance disclosure;
- future provider-candidate citation failure;
- artifact store write failure.

Errors must not include:

- run IDs;
- report IDs;
- side-effect IDs;
- approval IDs;
- paths;
- provider payloads;
- tokens;
- snippets;
- raw command output.

## 12. Storage And Artifact Posture

The implementation may write only through the supplied `WorkReportArtifactStore`.

It must not:

- write artifacts automatically from default executor methods;
- create filesystem artifacts outside the configured store;
- add persistence beyond existing store traits;
- add CLI rendering or export commands;
- change workflow schemas.

## 13. Test Plan

Future implementation tests should cover:

- existing artifact-capable executor success still writes an artifact;
- executor path now uses the generic integration helper behavior;
- workflow-derived high-assurance policy still composes by strictness;
- caller-disabled policy cannot weaken workflow-derived policy;
- artifact write failure preserves run/report result semantics;
- side-effect integrity failure preserves run/report result semantics;
- approval-linkage failure preserves run/report result semantics;
- high-assurance disclosure failure preserves run/report result semantics;
- no workflow events are appended by artifact write success or failure;
- no provider writes occur;
- no CLI output is emitted;
- Debug output remains bounded;
- existing executor, WorkReport, SideEffect, high-assurance, provider-write, and docs tests still pass.

If provider-candidate executor inputs are added later, tests must also cover:

- GitHub PR comment provider-candidate validation success;
- missing required GitHub proposed event fails before artifact write;
- unavailable provider-candidate IDs are not fabricated;
- GitHub provider payloads are not copied.

## 14. Proposed Implementation Sequence

1. Refactor `execute_with_report_artifact_and_side_effect_gates(...)` to call `write_report_artifact_with_explicit_integrations(...)` with `ReportArtifactWriteProviderIntegration::None`.
2. Preserve the existing `LocalExecutionWithReportArtifactResult` shape.
3. Add regression tests proving behavior is unchanged and the effective high-assurance policy still flows into the generic helper.
4. Run full validation.
5. Review.
6. Only after review, plan optional provider-candidate executor inputs.

## 15. Deferred Work

Deferred:

- provider-candidate executor inputs;
- GitHub PR comment live writes;
- provider mutation;
- runtime side-effect execution;
- automatic artifact writing;
- CLI artifact commands;
- schema changes;
- examples;
- hosted/distributed runtime;
- reasoning lineage;
- write-capable adapters;
- release posture changes.

## 16. Open Questions

- Should `LocalExecutionReportArtifactInputs` eventually expose a named effective policy type?
- Should provider-candidate integration inputs live inside artifact inputs or a separate executor request field?
- Should generic helper result details be exposed more directly in `LocalExecutionWithReportArtifactResult`, or should the existing result shape remain stable for now?
- Should future provider-candidate failures use provider-specific error codes or be mapped into a generic executor artifact namespace?

## 17. Final Recommendation

Recommended next implementation phase: executor artifact path uses generic report artifact write integration helper.

The implementation should be a narrow refactor of the explicit artifact-capable executor path only. It must not add default artifact behavior, provider writes, runtime side-effect execution, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 18. Validation

- `npm run check:docs`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783226015737855000-2 --phase planning`: passed.

Dogfood governance summary:

- workflow: `dg/d`;
- run: `run-1783226015737855000-2`;
- approval: `approval/run-1783226015737855000-2/planning-approved`;
- approval outcome: granted by delegated maintainer;
- terminal status: completed;
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
- out-of-kernel work: planning document edits, roadmap update, validation command, git/PR actions, and report posture were performed by the executor outside the kernel and disclosed here.
