# Governed Workflow Authoring Plan

Status: First model/helper slice implemented in [Governed Workflow Draft Proposal Implementation Report](../concepts/GOVERNED_WORKFLOW_DRAFT_PROPOSAL_IMPLEMENTATION_REPORT.md) and accepted in [Governed Workflow Draft Proposal Implementation Review](../concepts/GOVERNED_WORKFLOW_DRAFT_PROPOSAL_IMPLEMENTATION_REVIEW.md). The explicit CLI dry-run boundary is implemented in [Governed Workflow Authoring CLI Dry-Run Implementation Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_CLI_DRY_RUN_IMPLEMENTATION_REPORT.md), following [Governed Workflow Authoring CLI Dry-Run Plan](governed-workflow-authoring-cli-dry-run-plan.md), and accepted in [Governed Workflow Authoring CLI Dry-Run Implementation Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_CLI_DRY_RUN_IMPLEMENTATION_REVIEW.md). The explicit inactive file-output boundary is implemented in [Governed Workflow Authoring File Output Implementation Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_FILE_OUTPUT_IMPLEMENTATION_REPORT.md), following [Governed Workflow Authoring File Output Plan](governed-workflow-authoring-file-output-plan.md). Promotion and steward review are planned in [Governed Workflow Authoring Promotion And Steward Review Plan](governed-workflow-authoring-promotion-plan.md), the first preflight-only implementation slice is documented in [Governed Workflow Authoring Promotion Preflight Implementation Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_PROMOTION_PREFLIGHT_IMPLEMENTATION_REPORT.md) and accepted in [Governed Workflow Authoring Promotion Preflight Implementation Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_PROMOTION_PREFLIGHT_IMPLEMENTATION_REVIEW.md), and the steward-review boundary is planned in [Governed Workflow Authoring Steward Review Plan](governed-workflow-authoring-steward-review-plan.md). The first pure in-memory steward-review helper is implemented in [Governed Workflow Authoring Steward Review Helper Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_HELPER_REPORT.md) and accepted in [Governed Workflow Authoring Steward Review Helper Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_HELPER_REVIEW.md). The next bounded CLI preview surface is planned in [Governed Workflow Authoring Steward Review CLI Preview Plan](governed-workflow-authoring-steward-review-cli-preview-plan.md) and documented in [Governed Workflow Authoring Steward Review CLI Preview Plan Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_CLI_PREVIEW_PLAN_REPORT.md).

This plan remains the umbrella roadmap for governed workflow authoring. Current implemented slices provide preview output and explicit inactive draft file output only. They do not implement workflow registration, promotion, active workflow generation, command execution, local check execution, provider calls, schemas, examples, hosted behavior, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

`workflow-os first-run` can now inspect safe repository metadata, emit concrete review-only recommendations, attach bounded next-action hints, and show detail for an individual recommendation.

The next product question is how a maintainer or agent should turn a recommendation into a real governed workflow without making manual YAML authoring the only path and without silently mutating a repository.

Governed workflow authoring should eventually create reviewable draft workflow proposals from first-run recommendations. Those drafts must remain inactive until reviewed, validated, and explicitly accepted. The first implementation should not write workflow files automatically. It should begin with a model/helper that describes authoring obligations and proposed draft content in memory or preview output only.

## 2. Goals

- Reduce the gap between first-run recommendations and usable governed workflow specs.
- Preserve the product thesis that agents can move quickly while Workflow OS governs the boundary.
- Keep humans in stewardship roles: review, approve, reject, amend, promote, or retire workflow proposals.
- Make recommendation-to-workflow authoring deterministic and inspectable.
- Preserve safe metadata boundaries.
- Produce draft workflow proposals that are clearly inactive until accepted.
- Require owner, escalation, policy, evidence, checks, side-effect posture, and report/handoff posture before a draft can become active.
- Avoid raw source contents, raw command output, provider payloads, parser payloads, environment values, credentials, and token-like strings.
- Prepare future catalog governance without implementing catalog storage.

## 3. Non-Goals

Do not implement in this lane:

- automatic active workflow generation;
- automatic workflow registration or promotion;
- repository file writes;
- hidden edits to `workflows/`, `skills/`, `policies/`, or tests;
- local command execution;
- local check execution;
- provider calls;
- write-capable adapters;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- enterprise RBAC, IdP integration, paging, or escalation notifications;
- catalog persistence;
- release posture changes.

## 4. Current Inputs

The authoring lane should start from already-bounded first-run data:

- recommendation id;
- recommendation kind;
- target surface and ordinal;
- recommendation status;
- summary code;
- rationale codes;
- metadata-signal codes;
- spec-field coverage codes;
- ownership and escalation issue codes;
- next-action code;
- safe repo metadata labels and counts;
- governance field posture;
- check/evidence/side-effect/report posture.

It must not add new raw repository reads in the first implementation.

## 5. Authoring Boundary

Recommendation detail is not workflow authoring.

Governed authoring should be a separate step that makes draft obligations explicit:

- what workflow purpose is proposed;
- which owner and escalation fields must be filled;
- which policy gates are required or suggested;
- which evidence/check obligations are required or suggested;
- which side-effect posture is required;
- which final report or handoff obligations are required;
- what validation must pass before promotion;
- what the draft explicitly does not authorize.

The authoring output should be a proposal, not an active workflow. A proposal can be accepted only through a future explicit review/promotion path.

## 6. Candidate User Experience

Future UX should make the safe path feel obvious:

```sh
workflow-os first-run
workflow-os first-run --recommendation first_run.typescript_implementation
workflow-os author workflow --from-recommendation first_run.typescript_implementation --dry-run
workflow-os author workflow --from-recommendation first_run.typescript_implementation --output workflows/drafts/typescript-implementation.workflow.yml
```

Implemented first slices:

- introduce a preview/dry-run authoring surface;
- generate bounded proposed workflow sections in memory or text output;
- require explicit `--dry-run` for preview-only authoring;
- add explicit `--output workflows/drafts/<name>.workflow.yml` for one inactive draft file;
- do not register or activate workflows;
- show missing fields as obligations, not fabricated values.

The implemented CLI shape is `workflow-os author workflow`. It remains preview-stage and additive.

## 7. Draft Proposal Model

The future model should represent a draft proposal separately from an active workflow.

Candidate fields:

- proposal id;
- source recommendation id;
- source recommendation kind;
- proposed workflow id;
- proposed lifecycle status such as `draft`;
- proposed purpose code;
- required owner field;
- required escalation field;
- required policy gates;
- required evidence/check obligations;
- required side-effect posture;
- required report/handoff obligations;
- validation expectations;
- missing required authoring decisions;
- explicit non-goals;
- redaction metadata;
- sensitivity.

The proposal should be serializable only if serialization remains redaction-safe. Persistence remains out of scope until catalog/store planning.

## 8. Proposed Draft Content Policy

The first proposal slice should use bounded scaffolding, not project-specific claims.

Allowed content:

- stable recommendation ids and codes;
- known Workflow OS vocabulary;
- safe metadata labels such as `repo_metadata.typescript_detected`;
- common validation obligation labels such as `validation.npm_test_suggested`, without raw script bodies;
- placeholder owner/escalation obligations;
- side-effect posture values such as `none`, `skipped`, or `unsupported`;
- report/handoff obligation codes.

Forbidden content:

- raw source contents;
- raw `package.json` script bodies;
- raw dependency values;
- raw CI logs;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like strings;
- generated active workflow YAML. The implemented file-output slice writes inactive draft files under `workflows/drafts/` only.

## 9. Promotion And Activation Boundary

Creating a draft proposal must not activate governance.

Promotion should remain future and should require:

- successful project validation;
- explicit human or delegated maintainer approval;
- conflict checks against existing workflow ids and purpose/authority surfaces;
- owner and escalation completion;
- evidence/check posture review;
- side-effect posture review;
- report/handoff posture review;
- a final WorkReport or equivalent governed handoff.

Until promotion exists, draft proposals are advisory planning artifacts.

## 10. Conflict Handling

Workflow authoring should not create overlapping governance accidentally.

Future authoring should check:

- duplicate workflow ids;
- overlapping purposes;
- overlapping authority scope;
- overlapping side-effect or resource boundary;
- conflicting policy gates;
- conflicting approval posture;
- missing owner or escalation fields;
- stale lifecycle or supersession conflicts;
- unsafe dependency cycles.

The first authoring implementation may only disclose conflict checks as deferred if no catalog model exists yet.

## 11. Human And Enterprise Stewardship

Single local users may want mostly automated agent execution with standardized evidence, logs, and reports. Enterprise users need stewarded governance where admins or owners decide which gates are required.

Authoring should preserve that separation:

- local preview mode can propose low-friction workflows for review;
- enterprise posture should allow stewards to require owners, approvals, evidence, checks, side-effect posture, and reports before promotion;
- the kernel should not claim enterprise RBAC or IdP-backed authority before those systems exist.

## 12. Error Handling

Future implementation should fail closed when:

- the recommendation id is unknown;
- recommendation detail cannot be computed safely;
- a proposed workflow id would be invalid;
- required proposal fields are missing;
- proposal content would require raw payload copying;
- validation cannot determine whether the draft is safe.

Errors must use stable codes and must not echo raw ids, paths, source snippets, command bodies, provider payloads, parser payloads, credentials, or token-like values.

Candidate error codes:

- `cli.workflow_authoring.recommendation_not_found`;
- `cli.workflow_authoring.unsupported_recommendation_kind`;
- `cli.workflow_authoring.required_field_missing`;
- `cli.workflow_authoring.unsafe_payload_rejected`;
- `cli.workflow_authoring.conflict_check_deferred`.

## 13. Privacy And Redaction

Authoring must use bounded safe metadata only.

The authoring path should not print or persist raw:

- source files;
- manifest bodies;
- package script command bodies;
- lockfiles;
- dependency values;
- CI logs;
- provider payloads;
- issue/PR bodies;
- environment values;
- credentials or token-like values;
- existing agent instruction bodies.

If future output includes YAML previews, it must be generated from stable vocabulary and placeholders only, not copied repository payloads.

## 14. Test Plan

Future implementation tests should cover:

- known recommendation produces a draft proposal in memory or preview output;
- unknown recommendation fails closed without echoing the id;
- proposal remains inactive and does not register a workflow;
- no files are written in the first implementation;
- no runtime state is created;
- no commands are executed;
- no provider calls are made;
- draft includes owner/escalation obligations;
- draft includes evidence/check obligations;
- draft includes side-effect posture obligation;
- draft includes report/handoff obligation;
- raw package script bodies are not copied;
- dependency values are not copied;
- source contents are not copied;
- proposal validation rejects unsafe payloads;
- existing first-run recommendation detail tests still pass;
- existing validate, scaffold, runtime, docs, and security tests still pass.

## 15. Proposed Implementation Sequence

1. Add a bounded draft proposal model/helper for recommendation-to-workflow authoring, in memory only.
2. Add tests for proposal shape, required obligations, inactivity, and non-leakage.
3. Add an explicit dry-run/preview CLI path if the helper is accepted.
4. Review before any file-writing path.
5. Plan draft file output separately, including conflict handling and promotion semantics.
6. Add explicit inactive draft file output with path safety, conflict checks, no overwrite, no registration, no promotion, and no runtime state.
7. Review the file-output implementation before considering promotion, steward review, catalog storage, or active workflow generation.
8. Plan catalog/store integration separately.
9. Defer active workflow registration, automatic promotion, command execution, schemas, examples, hosted behavior, and writes.

## 16. Open Questions

- Should the first implementation expose a CLI command or only an internal helper?
- Should draft proposals have stable ids before catalog storage exists?
- Should proposed workflow ids be suggested or always user-supplied?
- How much YAML-like preview is useful before file writing is approved?
- Should local users be allowed to accept low-friction defaults while enterprise posture requires steward review?
- How should authoring proposals cite first-run evidence and future report artifacts?
- What is the smallest proposal that proves value without implying active governance?

## 17. Final Recommendation

The model/helper-only slice and explicit CLI dry-run slice are implemented and reviewed. The explicit inactive file-output slice is implemented and reviewed. Promotion and steward review are planned. Promotion preflight is implemented as a bounded non-mutating CLI inspection path.

Proceed next to governed workflow authoring promotion preflight implementation review.

Any future implementation should remain narrow unless separately approved: no workflow registration, no active promotion, no command execution, no provider calls, no runtime state, no schemas, no examples, no hosted behavior, no write-capable adapters, and no release posture changes.
