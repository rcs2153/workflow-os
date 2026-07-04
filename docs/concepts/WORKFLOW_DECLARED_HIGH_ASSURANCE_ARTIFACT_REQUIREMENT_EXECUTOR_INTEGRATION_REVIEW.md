# Workflow-Declared High-Assurance Artifact Requirement Executor Integration Review

## 1. Executive Verdict

Needs blocker fixes.

The implementation correctly moves workflow-declared high-assurance report artifact requirements into the explicit artifact-capable executor path and preserves the conservative default executor posture. However, review found two blocker issues at the enforcement boundary:

- the artifact-capable validation posture is project-wide rather than scoped to the selected workflow being executed through the artifact-capable path;
- rehydrated artifact-path runs derive the artifact gate policy from current project files rather than verifying the current loaded workflow still matches the immutable run identity/spec hash.

Both issues can create false confidence about which declared requirements are actually enforced.

## 2. Scope Verification

The phase mostly stayed within the approved executor artifact-path integration scope.

Implemented within scope:

- capability-aware validation entry points;
- artifact-capable executor path validation;
- workflow-declared high-assurance artifact gate derivation;
- strictness composition between caller-supplied and workflow-declared artifact policies;
- artifact gate enforcement before artifact write;
- default executor path preservation;
- tests and documentation.

No accidental implementation found for:

- automatic report generation on default executor paths;
- automatic artifact writing on default executor paths;
- CLI artifact behavior;
- example updates;
- new workflow schema fields;
- TypeScript SDK changes;
- runtime config;
- workflow-declared high-assurance approval controls;
- RBAC, IdP, quorum approval, or revocation enforcement;
- approval evidence attachment;
- workflow event or audit projection for artifact derivation;
- side-effect execution;
- write-capable adapters;
- hosted/distributed runtime;
- reasoning lineage;
- release posture changes.

The scope issue is not feature broadening. It is that the artifact-capable validation capability is broader than the executor path it is intended to justify.

## 3. Validation Capability Assessment

Default validation remains conservative. `ProjectValidationCapability::Default` still rejects stronger workflow-declared report artifact requirements with:

```text
validation.workflow.report_artifact_requirement.runtime_not_enforced
```

That is correct.

The blocker is the `ProjectValidationCapability::ReportArtifactCapable` shape. It currently causes `validate_report_artifact_requirements(...)` to return early for every workflow in the loaded project. The explicit artifact path only executes one selected workflow, but the validation capability makes all workflows in the project appear artifact-capable for validation purposes.

That mismatch matters because a project can contain multiple workflows. If workflow A is executed through the artifact-capable path, workflow B can also declare high-assurance artifact requirements without validation errors in that validation pass, even though workflow B is not being executed or enforced.

The capability should be scoped to the selected workflow or otherwise fail closed for unrelated workflows.

## 4. Executor Integration Assessment

The fresh-run artifact path is directionally correct.

For a new run, `execute_for_report_artifact_path(...)`:

- validates with the artifact-capable posture;
- prepares an execution plan;
- derives the workflow-declared artifact policy from the selected workflow;
- evaluates pre-run policy;
- appends run start;
- executes steps;
- returns the run and derived policy.

Default `execute(...)` and `execute_with_report(...)` continue to use default validation and do not gain artifact behavior.

The blocker is the duplicate/rehydrated-run path. If events already exist for the requested run ID, the function returns:

```text
rehydrated run + workflow_report_artifact_policy_for_request(request)
```

That helper reloads the current project files and derives policy from the currently loaded workflow definition. It does not verify that the current workflow definition/version/content hash matches the immutable workflow identity captured when the run was created.

Workflow runs already carry immutable `workflow_version` and `spec_content_hash` in `WorkflowRunIdentity`. The artifact policy used for a rehydrated run must not be silently re-derived from changed YAML unless the loaded workflow is proven to match the run identity.

## 5. Policy Composition Assessment

The strictness composition rule is correct:

```text
effective_policy = stricter(caller_policy, workflow_declared_policy)
```

Callers can request a stricter policy, but cannot weaken a workflow-declared policy. This is the right direction for high-assurance artifact gates.

The issue is not the composition function. The issue is whether the workflow-declared policy being composed is scoped to the right workflow and the right immutable workflow definition for the run.

## 6. Runtime Semantics And Rehydration Assessment

The implementation preserves workflow pass/fail semantics:

- execution failure before a run exists returns `Err`;
- artifact gate failure after a run/report exists is report/artifact-scoped;
- artifact gate failure writes no partial artifact;
- artifact gate failure appends no workflow events.

However, the rehydration path needs a fail-closed identity check before using current project YAML to derive artifact policy for an existing run.

Required fix shape:

- load the selected workflow;
- compare current loaded workflow identity to `run.snapshot.identity.workflow_id`, `workflow_version`, `schema_version`, and `spec_content_hash`;
- fail closed with a stable non-leaking error if the current workflow no longer matches the run identity;
- only derive artifact policy from the loaded workflow after that match is proven.

An alternative durable fix is to persist or event-project the derived artifact policy at run creation, then reuse that immutable policy for rehydrated artifact paths. That is broader and should be planned separately if chosen.

## 7. Privacy And Redaction Assessment

No privacy or redaction blocker was found.

The integration reuses existing report and artifact constructors. It does not copy raw provider payloads, raw report payloads outside validated model fields, approval payloads, workflow YAML contents, command output, parser payloads, file contents, environment variable values, credentials, authorization headers, private keys, or token-like values.

Errors reviewed are stable and non-leaking in intent. The blocker fixes should preserve that posture and avoid including raw paths, YAML snippets, hashes, tokens, or caller payloads in error messages.

## 8. Test Quality Assessment

Current tests cover important happy and failure paths:

- default `execute(...)` rejects workflow-declared artifact enforcement posture;
- default `execute_with_report(...)` rejects workflow-declared artifact enforcement posture;
- artifact-capable path derives workflow-declared gate policy;
- disabled caller policy cannot erase workflow-declared disclosure requirement;
- workflow-declared validated/fail-closed requirement writes when valid high-assurance disclosure is supplied;
- caller policy stricter than workflow declaration wins;
- artifact gate failures preserve run/report and write no artifact;
- artifact gate failures append no events;
- existing `workflow-core` tests pass.

Missing blocker-level tests:

- a multi-workflow project where the selected workflow is artifact-capable but an unrelated workflow declares high-assurance artifact requirements must not be globally excused by artifact-capable validation;
- a rehydrated run whose current workflow YAML has changed to a weaker requirement must fail closed or preserve the original run policy rather than weakening the artifact gate;
- a rehydrated run whose current workflow YAML has changed to a different content hash/version/schema must fail closed before artifact write;
- the rehydration mismatch error must use a stable code and must not leak raw YAML, file paths, hashes, report text, or secret-like values.

## 9. Documentation Review

Docs correctly say:

- workflow-declared high-assurance artifact requirements are enforced only by the explicit artifact-capable path;
- default validation and default executor paths remain conservative;
- automatic report generation is not implemented;
- automatic artifact writing from default paths is not implemented;
- CLI artifact behavior is not implemented;
- examples are not updated;
- new schema fields are not added;
- side-effect execution and writes remain unsupported;
- hosted/distributed runtime, reasoning lineage, and release posture changes remain unsupported.

Documentation should be updated during the blocker fix to state that artifact-capable validation is selected-workflow scoped and that rehydrated artifact paths fail closed on workflow identity/spec mismatch.

## 10. Blockers

1. Artifact-capable validation is project-wide rather than selected-workflow scoped.

   Action: change the validation capability to carry the selected `WorkflowId`, or add an artifact-path validation helper that only relaxes `report_artifact_requirements` for the workflow being executed. All unrelated workflows must retain default conservative validation.

2. Rehydrated artifact-path policy derivation can use mutable current project files without proving they match the run identity.

   Action: before deriving artifact policy for an existing run, compare the loaded workflow identity/version/schema/spec hash against `run.snapshot.identity`. Fail closed with a stable non-leaking error on mismatch, or persist the derived policy at run creation and reuse that immutable policy.

## 11. Non-Blocking Follow-Ups

- Consider whether derived artifact policy should be stored in a workflow event or artifact metadata during a later phase.
- Consider a small helper for comparing a loaded workflow spec to `WorkflowRunIdentity`, since similar checks already exist in side-effect and hook contexts.
- Document the artifact-capable validation capability as a narrow execution-context capability, not a general project validation mode.

## 12. Recommended Next Phase

Recommended next phase: blocker fix.

The implementation is close, but this is a governance enforcement boundary. The next phase should fix scoped validation and rehydrated-run identity matching before expanding high-assurance approval controls, artifact automation, CLI behavior, write-capable adapters, or additional runtime composition.

## 13. Validation

Implementation phase validation reviewed:

- `cargo test -p workflow-core --test local_executor` passed.
- `cargo test -p workflow-core` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.
- `git diff --check` passed.

Additional review-phase validation:

- `npm run check:docs` passed.
- `git diff --check` passed.

## 14. Dogfood Governance

This review phase is governed by the local Workflow OS dogfood runner.

- workflow phase: review
- workflow ID: `dg/review`
- run ID: `run-1783142430580950000-2`
- approval ID: `approval/run-1783142430580950000-2/review-scope-approved`
- approval outcome: approved by the maintainer before review work continued
- close status: completed
- event summary: 39 total events, 1 approval, 0 retries, 0 escalations
- event kinds: `ApprovalGranted`, `ApprovalRequested`, `PolicyDecisionRecorded`, `RunCompleted`, `RunCreated`, `RunResumed`, `RunStarted`, `RunValidated`, `SkillInvocationRequested`, `SkillInvocationStarted`, `SkillInvocationSucceeded`, `StepScheduled`
- validation summary: docs check and whitespace check passed; implementation phase full Rust workspace validation was reviewed from the phase report

The dogfood runner coordinated governance only. Review writing, code inspection, and validation commands were performed by the executor.
