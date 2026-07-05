# Executor Provider-Candidate Report Artifact Integration Plan

Status: Implemented in
[Executor Provider-Candidate Report Artifact Integration Report](../concepts/EXECUTOR_PROVIDER_CANDIDATE_REPORT_ARTIFACT_INTEGRATION_REPORT.md).
This plan did not authorize runtime provider writes.

## 1. Executive Summary

The artifact-capable executor path now uses the generic report artifact write
integration helper with `ReportArtifactWriteProviderIntegration::None`.

The next question is how an explicit caller should provide provider-candidate
validation context to that executor path. The first provider candidate remains
GitHub pull request comment reporting, but this plan still does not authorize
live GitHub comment creation, provider writes, runtime side-effect execution,
automatic artifact writing, CLI mutation behavior, schemas, examples, hosted
behavior, reasoning lineage, or release posture changes.

The recommended next implementation is a narrow input-model expansion only:
allow `execute_with_report_artifact_and_side_effect_gates(...)` callers to pass
explicit provider-candidate context that is validated before artifact write.

## 2. Goals

- Add an explicit provider-candidate input shape for the artifact-capable
  executor path.
- Keep provider-candidate validation opt-in and caller-supplied.
- Reuse `ReportArtifactWriteProviderIntegration`.
- Preserve existing executor result shape.
- Preserve workflow-derived high-assurance artifact gate strictness.
- Validate GitHub PR comment provider-candidate context before artifact write
  when supplied.
- Require stable IDs and accepted proposed side-effect event proof where policy
  requires it.
- Avoid fabricating provider IDs, citations, evidence references, or events.
- Keep provider writes out of scope.
- Keep failures structured, bounded, and non-leaking.

## 3. Non-Goals

Do not implement:

- provider mutation;
- live GitHub PR comment creation;
- runtime side-effect execution;
- automatic artifact writing;
- automatic report generation;
- CLI mutation behavior;
- workflow schema changes;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current State

Implemented:

- `execute_with_report_artifact_and_side_effect_gates(...)`;
- explicit local report artifact generation and write path;
- workflow-declared high-assurance artifact requirement derivation;
- strictness composition between caller policy and workflow-derived policy;
- `write_report_artifact_with_explicit_integrations(...)`;
- `ReportArtifactWriteProviderIntegration::None`;
- `ReportArtifactWriteProviderIntegration::GitHubPullRequestComment`;
- GitHub PR comment provider-candidate validation helper;
- report artifact citation validation for proposed GitHub PR comment side
  effects.

Not implemented:

- passing provider-candidate context through executor artifact inputs;
- live GitHub comment creation;
- runtime side-effect execution;
- automatic artifact generation or persistence from default executor paths.

## 5. Recommended Input Shape

Extend `LocalExecutionReportArtifactInputs` with an optional provider-candidate
field such as:

```rust
pub provider_integration: Option<LocalExecutionReportArtifactProviderIntegrationInputs>
```

Add a bounded enum:

```rust
pub enum LocalExecutionReportArtifactProviderIntegrationInputs {
    GitHubPullRequestComment {
        side_effect_id: SideEffectId,
        workflow_events: Vec<SideEffectWorkflowEvent>,
        citation_policy: GitHubPullRequestCommentReportArtifactCitationPolicy,
    },
}
```

The initial implementation may choose a borrowed or owned shape according to
existing executor input conventions, but the caller must provide all provider
candidate context explicitly.

## 6. Mapping To Generic Helper

The executor path should map:

- `None` to `ReportArtifactWriteProviderIntegration::None`;
- `GitHubPullRequestComment { ... }` to
  `ReportArtifactWriteProviderIntegration::GitHubPullRequestComment { ... }`.

The executor should still pass the already-composed effective high-assurance
artifact policy to `ReportArtifactWriteIntegrationInput`.

The mapping must not read hidden global state, query GitHub, inspect PRs, create
events, create evidence references, or infer missing IDs.

## 7. Citation And Event Policy

For the GitHub PR comment provider candidate:

- the expected `SideEffectId` must be caller-supplied;
- workflow events must be caller-supplied if citation policy requires accepted
  proposed-event proof;
- citation policy must be explicit;
- missing required proposed-event proof must fail before artifact write;
- approval-linkage requirements must remain enforced when configured;
- missing IDs must not be fabricated;
- unavailable provider references should remain absent or explicit in report
  text rather than invented.

## 8. Workflow Semantics

The executor artifact path must preserve existing semantics:

- execution failure before a run exists still returns `Err`;
- report-generation failure after a run exists remains inside the result;
- artifact-write or provider-candidate validation failure after a report exists
  remains inside the result;
- workflow run status is not changed by artifact success or failure;
- no workflow events are appended;
- no audit or observability events are emitted;
- no side effects are executed.

## 9. Failure Behavior

Provider-candidate validation failures should appear as artifact write errors
inside `LocalExecutionWithReportArtifactResult`.

Failures must use stable non-leaking codes for:

- missing expected side-effect record;
- side-effect identity mismatch;
- missing accepted proposed-event proof;
- missing required citation;
- approval-linkage failure;
- high-assurance disclosure failure;
- artifact store write failure.

Errors must not include:

- raw provider payloads;
- GitHub PR bodies or comment bodies;
- command output;
- CI logs;
- spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, private keys, or token-like values;
- raw IDs when the existing error boundary treats them as sensitive.

## 10. Privacy And Redaction

The executor path must continue to rely on validated WorkReport, artifact,
side-effect, provider-candidate, and citation constructors.

Provider-candidate inputs may include stable IDs and bounded references only.
They must not include raw comment bodies, PR bodies, provider response payloads,
or live credentials.

Debug output for any new input type must be bounded and redaction-safe.

## 11. Test Plan

Future implementation tests should cover:

- existing `None` provider path remains unchanged;
- supplied GitHub PR comment provider candidate maps to the generic helper;
- accepted proposed-event proof allows artifact write when citation policy
  requires it;
- missing proposed-event proof fails before artifact write;
- provider-candidate validation failure preserves run/report result semantics;
- workflow event history is not mutated;
- artifact store is not written on provider-candidate validation failure;
- approval-linkage requirements still compose with provider-candidate checks;
- high-assurance disclosure requirements still compose with provider-candidate
  checks;
- Debug output for new executor input does not leak IDs, payloads, paths, or
  secret-like values;
- no live provider calls occur;
- existing executor, WorkReport, provider-write, side-effect, validation, and
  runtime tests still pass.

## 12. Proposed Implementation Sequence

1. Add the bounded provider-candidate executor input enum.
2. Add mapping from executor inputs to `ReportArtifactWriteProviderIntegration`.
3. Keep `None` as the default behavior.
4. Add focused executor tests for the GitHub PR comment provider-candidate
   branch.
5. Validate with full workspace checks.
6. Review before any broader provider candidate or CLI exposure.

## 13. Open Questions

- Should provider-candidate inputs be owned values or borrowed values?
- Should provider-candidate inputs be nested under artifact inputs or report
  inputs?
- Should missing proposed-event proof use the existing citation error code or a
  new executor-adjacent wrapper code?
- Should future provider candidates share a common citation policy trait, or
  remain enum-specific?
- How should this evolve when workflow-declared provider artifact requirements
  exist?

## 14. Final Recommendation

Proceed to a narrow implementation of explicit provider-candidate executor
inputs for the artifact-capable executor path.

The first implementation should support only the GitHub PR comment provider
candidate branch and must not create comments, call GitHub, execute side
effects, expose CLI behavior, update schemas, or make artifact generation
automatic.

## 15. Planning Validation

- `npm run check:docs`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783227494137116000-2 --phase planning`: passed.

Governed planning:

- workflow: `dg/d`;
- run: `run-1783227494137116000-2`;
- approval: `approval/run-1783227494137116000-2/planning-approved`;
- approval outcome: granted by delegated maintainer;
- phase closeout: completed;
- events: 39 total, 1 approval, 0 retries, 0 escalations.
