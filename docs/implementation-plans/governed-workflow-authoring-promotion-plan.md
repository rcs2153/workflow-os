# Governed Workflow Authoring Promotion And Steward Review Plan

Status: Promotion/steward review planned; first preflight-only CLI slice implemented in [Governed Workflow Authoring Promotion Preflight Implementation Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_PROMOTION_PREFLIGHT_IMPLEMENTATION_REPORT.md) and accepted in [Governed Workflow Authoring Promotion Preflight Implementation Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_PROMOTION_PREFLIGHT_IMPLEMENTATION_REVIEW.md). The focused steward-review boundary is planned in [Governed Workflow Authoring Steward Review Plan](governed-workflow-authoring-steward-review-plan.md), documented in [Governed Workflow Authoring Steward Review Plan Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_PLAN_REPORT.md), and its first pure in-memory helper is accepted in [Governed Workflow Authoring Steward Review Helper Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_HELPER_REVIEW.md). The next bounded CLI preview surface is planned in [Governed Workflow Authoring Steward Review CLI Preview Plan](governed-workflow-authoring-steward-review-cli-preview-plan.md) and documented in [Governed Workflow Authoring Steward Review CLI Preview Plan Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_CLI_PREVIEW_PLAN_REPORT.md).

This plan follows the accepted inactive file-output implementation reviewed in [Governed Workflow Authoring File Output Implementation Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_FILE_OUTPUT_IMPLEMENTATION_REVIEW.md). It defines how Workflow OS should eventually move from review-only draft workflow files to explicitly reviewed, validated, steward-approved active workflow specs.

The first implementation adds `workflow-os author workflow preflight --draft workflows/drafts/<name>.workflow.yml` as a bounded promotability inspection command. It does not implement workflow promotion, active workflow registration, file movement, command execution, provider calls, runtime state creation, schemas, examples, hosted behavior, write-capable adapters, or release posture changes.

## 1. Executive Summary

Workflow OS can now write explicit inactive draft workflow files under `workflows/drafts/`.

The next product question is not how to generate more YAML. The next product question is how a draft becomes trusted enough to govern real work.

Promotion must be a governed boundary. It should require deterministic validation, owner and escalation completion, conflict checks, policy/evidence/check posture review, side-effect and report posture review, steward or delegated maintainer approval, and a clear record of what changed.

The first implementation after this plan should be a promotion preflight or validator helper only. It should inspect a draft and report whether it is promotable without moving files, registering workflows, creating runtime state, executing commands, or writing active specs.

## 2. Goals

- Define the boundary between inactive draft files and active workflow specs.
- Keep workflow promotion explicit, deterministic, and reviewable.
- Require owner and escalation posture before activation.
- Require policy, evidence, validation/check, side-effect, approval, audit, and report posture before activation.
- Detect id, path, purpose, authority, and surface conflicts before activation.
- Preserve existing active workflow behavior.
- Keep drafts review-only until promotion is explicitly approved.
- Support local users and future steward/admin workflows without assuming enterprise infrastructure exists.
- Prepare for future workflow catalog/store work without implementing it.
- Preserve the principle: Agent executes. Workflow OS governs.

## 3. Non-Goals

Do not implement in this phase:

- workflow promotion;
- active workflow registration;
- file movement from `workflows/drafts/` to `workflows/`;
- mutation of workflow specs;
- workflow catalog persistence;
- runtime state creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- RBAC, IdP, department-level admin UI, paging, or notification systems;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. Current Boundary

Current implemented authoring capabilities:

- `workflow-os first-run` emits review-only recommendations.
- `workflow-os first-run --recommendation <id>` shows bounded recommendation detail.
- `workflow-os author workflow --from-recommendation <id> --dry-run` previews authoring obligations.
- `workflow-os author workflow --from-recommendation <id> --output workflows/drafts/<name>.workflow.yml` writes one inactive draft file for review.

Current preflight capability:

- `workflow-os author workflow preflight --draft workflows/drafts/<name>.workflow.yml` inspects one inactive draft in isolation, reports bounded blocker/warning codes, checks active workflow id conflicts, and validates the draft as a candidate without moving files, registering workflows, creating runtime state, executing commands, or calling providers.

Current non-capabilities:

- no active registration from drafts;
- no promotion command;
- no steward approval model for drafts;
- no catalog/store-backed proposal state;
- no typed draft lifecycle schema value;
- no workflow-declared authoring requirements;
- no automatic workflow generation or mutation.

## 5. Promotion Definition

Promotion means a reviewed draft becomes an active workflow spec that the current project loader and runtime may validate and execute.

Promotion is not:

- writing an inactive draft file;
- previewing authoring obligations;
- showing a recommendation;
- passing generic YAML parsing;
- an agent deciding the workflow is good enough;
- a model self-review.

Promotion should be treated as a governed change to the repository's control plane.

## 6. Steward Review Model

For v0 local use, the steward can be a human maintainer or explicitly delegated maintainer.

Future enterprise stewardship may include:

- workflow owners;
- escalation contacts;
- security or governance reviewers;
- department or workspace administrators;
- policy stewards;
- approval authorities;
- audit/release stewards.

The local kernel should not require enterprise infrastructure to be useful. It should still expose the same questions:

- Who owns this workflow?
- Who gets escalated when it fails?
- What authority does it grant?
- What evidence does it require?
- What checks must pass?
- What side effects are allowed or forbidden?
- What approvals are required?
- What report/handoff must close the work?
- What existing workflows overlap with it?

## 7. Promotion Preconditions

A future promotion preflight should require:

- valid draft path under `workflows/drafts/`;
- valid proposed active workflow id;
- no duplicate active workflow id;
- no unsafe filename or secret-like id material;
- owner field completed with non-placeholder posture;
- escalation posture completed or explicitly marked not applicable by policy;
- workflow purpose bounded and non-empty;
- active triggers declared;
- active steps declared;
- every skill reference resolves;
- every policy reference resolves;
- policy effects are supported in their reference context;
- approval requirements and approval policies are aligned;
- evidence/check obligations are represented;
- side-effect posture is explicit;
- audit/observability posture is explicit;
- report/handoff posture is explicit;
- autonomy level remains within supported local boundary;
- validation diagnostics have no errors.

## 8. Conflict Checks

Promotion should fail closed on:

- duplicate workflow id;
- duplicate or overlapping trigger id where deterministic checks exist;
- conflicting workflow purpose or authority surface where deterministic labels exist;
- conflicting side-effect authority;
- incompatible autonomy level;
- missing owner or escalation posture;
- missing policy/evidence/check/report posture;
- unsupported policy effects;
- unresolved skill/policy references;
- unsafe path, id, owner, escalation, or note values;
- draft content that would require copying raw source, command, provider, or secret payloads.

For the first implementation, id/path/reference checks are required. Purpose/authority overlap can start as explicit warning/disclosure if deterministic taxonomy is not ready.

## 9. Candidate CLI Shape

First implementation should be preflight-only:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --dry-run
```

Alternative if the command feels too action-oriented:

```sh
workflow-os author workflow preflight \
  --draft workflows/drafts/<name>.workflow.yml
```

The first slice should not move files. It should return a promotability report with required blockers, warnings, missing fields, conflict checks, validation posture, and next action.

Any future write-capable promotion command must be planned and reviewed separately.

## 10. Validation Boundary

Promotion preflight should use existing structured loaders and validators where possible.

It should not:

- parse YAML with ad hoc string inspection;
- execute commands;
- run local checks;
- call providers;
- inspect raw source contents;
- create runtime state;
- append events.

If current validation cannot validate nested drafts directly, the preflight helper should load the draft in an isolated candidate context and validate the candidate without registering it in the project bundle.

## 11. Evidence, Checks, And Reports

Promotion should require explicit posture for:

- evidence references expected from future runs;
- validation/check obligations;
- local check execution posture;
- side-effect disclosure posture;
- report/handoff closure posture.

Promotion should not require evidence that only exists after a workflow run. It should require that future evidence obligations are declared and reviewable.

## 12. Approval And Ownership

Promotion should require approval when:

- a draft becomes active;
- owner or escalation posture is changed;
- policy gates or approval requirements change;
- side-effect authority changes;
- evidence/check/report obligations are weakened;
- autonomy level changes.

Local v0 can model this as explicit delegated maintainer approval. Enterprise stewardship can later decide which approvals require humans, agents, groups, or admin policies.

## 13. Privacy And Redaction

Promotion preflight must not copy:

- raw source contents;
- manifest bodies;
- package script command bodies;
- dependency values;
- lockfile contents;
- CI logs;
- provider payloads;
- issue or pull request bodies;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like strings;
- existing agent instruction bodies.

Errors must use stable codes and avoid echoing unsafe path, id, owner, escalation, note, or raw payload values.

## 14. Error Handling

Promotion preflight should return structured, non-leaking diagnostics.

Recommended error classes:

- draft path rejected;
- draft file missing;
- draft parse failed;
- draft validation failed;
- required promotion field missing;
- active workflow id conflict;
- unsupported policy effect;
- unresolved reference;
- unsafe payload rejected;
- promotion blocked by explicit non-goal.

The first implementation should fail closed on errors and should not emit partial active workflow content.

## 15. Test Plan

Future tests should cover:

- valid completed draft candidate produces promotable preflight result;
- incomplete generated draft is not promotable;
- missing owner blocks promotion;
- missing escalation posture blocks or warns according to policy;
- missing triggers or steps block promotion;
- unresolved skill reference blocks promotion;
- unresolved policy reference blocks promotion;
- duplicate active workflow id blocks promotion;
- unsafe draft path blocks without leakage;
- secret-like id, owner, escalation, or note blocks without leakage;
- unsupported policy effect blocks promotion;
- side-effect authority conflict is disclosed or blocked;
- report/handoff posture is required;
- preflight creates no runtime state;
- preflight writes no files;
- preflight executes no commands;
- preflight calls no providers;
- docs check passes.

## 16. Proposed Implementation Sequence

1. Add a pure promotion preflight helper that accepts a draft path and current project bundle, returns a bounded promotability result, and writes nothing.
2. Add CLI dry-run/preflight surface for the helper.
3. Add focused tests for blockers, warnings, non-leakage, and non-mutation.
4. Review.
5. Plan steward approval integration.
6. Plan active promotion/file movement separately.
7. Defer catalog/store-backed workflow proposals until the local preflight boundary is reviewed.

## 17. Open Questions

- Should the command be named `promote --dry-run`, `preflight`, or something else?
- Should generated drafts become a separate proposal artifact instead of workflow-shaped YAML?
- Should Workflow OS add a typed `draft` lifecycle status before promotion?
- How strict should owner/escalation requirements be for single-user local repos?
- How should enterprise stewards override or harden local promotion policy?
- Which conflict checks can be deterministic now versus warning-only?
- Should promotion approval require a WorkReport artifact once report artifacts are stable?
- How should promoted workflows preserve source recommendation provenance?

## 18. Final Recommendation

Proceed next to governed workflow authoring promotion preflight implementation.

The next implementation should be model/helper and CLI preflight only. It must not move files, register workflows, activate drafts, create runtime state, execute commands, call providers, add schemas, add examples, enable writes, or change release posture.
