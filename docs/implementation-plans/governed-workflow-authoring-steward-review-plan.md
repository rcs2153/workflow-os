# Governed Workflow Authoring Steward Review Plan

Status: Planning only. No steward-review promotion command, active workflow registration, file movement, runtime state, schemas, examples, hosted behavior, writes, or release posture changes are implemented by this plan.

This plan follows the accepted preflight-only implementation reviewed in [Governed Workflow Authoring Promotion Preflight Implementation Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_PROMOTION_PREFLIGHT_IMPLEMENTATION_REVIEW.md). It defines the next boundary after `workflow-os author workflow preflight --draft ...`: how a preflight-passing inactive draft should be reviewed by a steward or delegated maintainer before any future active-promotion implementation exists.

## 1. Executive Summary

Workflow OS can now generate inactive workflow drafts and inspect draft promotability through a non-mutating preflight command.

The next boundary is steward review: deciding whether a draft should be allowed to become an active workflow spec in a future phase.

Steward review must not be a vague model opinion or silent file move. It must be a bounded approval decision over a specific draft, a specific preflight result, a specific ownership/escalation posture, and specific governance obligations.

This plan does not implement steward review. It does not move files, register workflows, activate drafts, create runtime state, execute commands, call providers, add schemas, add examples, enable writes, or change release posture.

## 2. Goals

- Define the steward-review boundary after preflight and before active promotion.
- Preserve the non-mutating preflight boundary.
- Make approval context explicit enough for a human or delegated maintainer to decide.
- Require owner and escalation completion before promotion can be approved.
- Require policy, evidence, validation/check, side-effect, audit, and report posture review.
- Make conflicts visible before activation.
- Preserve local-single-user and future enterprise-steward paths.
- Prepare a small future implementation prompt without authorizing it here.

## 3. Non-Goals

This plan does not implement:

- active workflow promotion;
- workflow registration;
- file movement from `workflows/drafts/` to `workflows/`;
- mutation of active workflow specs;
- workflow catalog persistence;
- runtime state creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- workflow-declared steward configuration;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- RBAC, IdP, admin UI, paging, or notification systems;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. Steward Review Boundary

Steward review is the decision boundary between:

- an inactive draft workflow file under `workflows/drafts/`; and
- a future active workflow spec under `workflows/`.

For v1, steward review should remain explicit and local. It should accept a draft path and a fresh preflight result. It should not infer approval from the existence of a draft, a passing validation run, an agent recommendation, or a green local check.

Approval should mean:

- this exact draft is acceptable for future active promotion;
- the reviewer has seen the preflight result and governance posture;
- owner and escalation posture are acceptable;
- required governance obligations are understood;
- no known conflict blocks activation.

Approval should not mean:

- the workflow has already been promoted;
- commands have been run;
- checks have passed unless cited separately;
- side effects are allowed;
- writes are enabled;
- runtime execution has occurred;
- future edits to the draft are also approved.

## 5. Required Review Inputs

A future steward-review helper should require:

- relative draft path under `workflows/drafts/`;
- candidate workflow id;
- draft content hash;
- preflight status;
- preflight blocker codes;
- preflight warning codes;
- owner and maintainer posture;
- escalation contact posture;
- policy requirements declared by the draft;
- approval policies declared by the draft;
- local check or validation obligations declared by the draft, if any;
- evidence/report posture declared by the draft, if any;
- side-effect posture declared by the draft, if any;
- active workflow id/path conflict status;
- reviewer actor;
- approval decision;
- bounded approval reason.

Review must fail closed if the draft changed after preflight unless a new preflight is supplied.

## 6. Approval Card Model

The future steward-review surface should produce a human-readable approval card before approval.

The card should include:

- draft path;
- candidate workflow id;
- draft content hash;
- preflight result summary;
- blocker/warning codes;
- owner and escalation summary;
- policy and approval summary;
- validation/check summary;
- evidence/report summary;
- side-effect summary;
- active workflow conflict summary;
- what approval allows;
- what approval does not allow;
- required next action after approval;
- known limitations.

The card must not copy raw workflow YAML, raw source contents, command output, provider payloads, secrets, token-like strings, or private absolute paths.

## 7. Decision Semantics

Steward review should support at least:

- `approved_for_promotion`: the draft can move to a future promotion step if unchanged;
- `denied`: promotion must not proceed;
- `needs_changes`: draft should remain inactive and be revised;
- `deferred`: no decision yet.

Only `approved_for_promotion` should permit a future promotion command to proceed.

Denied, needs-changes, and deferred decisions must not mutate active workflows.

## 8. Freshness And Idempotency

Steward approval must bind to:

- draft path;
- candidate workflow id;
- draft content hash;
- preflight result;
- reviewer actor;
- decision timestamp or supplied timestamp.

If the draft content hash changes after approval, the approval is stale and must not authorize promotion.

Repeated review of the same unchanged draft should be idempotent from a state perspective until a separate storage model exists. In the first implementation, the helper can be pure/in-memory and return review output without persisting anything.

## 9. Local And Enterprise Posture

For a single local user, steward review may be performed by the same delegated maintainer or agent-supervised operator if the governance profile allows it.

For enterprise use, the same boundary should later support:

- admin/steward-defined reviewer requirements;
- separation of author and approver;
- department/team ownership;
- escalation rules;
- workflow lifecycle governance;
- audit retention;
- stronger approval evidence.

This plan does not implement enterprise stewardship. It keeps the review vocabulary compatible with that future path.

## 10. Error Handling

Future steward review should fail closed when:

- draft path is unsafe;
- draft is missing;
- draft cannot be parsed;
- preflight has blockers;
- preflight result is missing;
- preflight result is stale;
- required owner/escalation fields are incomplete;
- reviewer actor is missing or invalid;
- approval reason is missing, unbounded, or secret-like;
- decision is unsupported;
- active workflow conflicts are present.

Errors must use stable codes and must not echo raw YAML, private paths, secret-like values, provider payloads, parser payloads, command output, or source snippets.

## 11. Privacy And Redaction

Review output should use bounded summaries and codes.

It must not copy:

- raw draft YAML;
- raw source contents;
- raw package scripts;
- raw CI logs;
- raw command output;
- raw provider payloads;
- raw parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like strings;
- private absolute paths.

Draft paths should remain relative to the project. Content hashes may be shown because they identify exact reviewed content without exposing the content itself.

## 12. Test Plan

Future implementation tests should cover:

- preflight-passing draft can produce a review approval card;
- preflight-blocked draft cannot be approved for promotion;
- missing preflight result fails closed;
- stale preflight/content hash fails closed;
- unsafe draft path fails without leaking path values;
- missing owner posture fails closed;
- missing escalation posture fails closed;
- duplicate active workflow id fails closed;
- denied decision does not authorize promotion;
- needs-changes decision does not authorize promotion;
- deferred decision does not authorize promotion;
- approval reason rejects secret-like values;
- approval card does not copy raw YAML;
- approval card does not copy command output or provider payloads;
- review helper writes no files and creates no runtime state;
- existing preflight tests continue to pass;
- docs check passes.

## 13. Proposed Implementation Sequence

Recommended small future phases:

1. Add a pure steward-review result/helper model that accepts explicit draft/preflight/reviewer inputs and writes nothing.
2. Add a CLI preview command for steward review that prints the approval card and decision result without persisting approval.
3. Add focused redaction and stale-hash tests.
4. Review the steward-review helper.
5. Plan active promotion separately.
6. Only after review, implement active promotion as an explicit file-movement/registration boundary.

## 14. Deferred Work

- Active workflow promotion.
- File movement from `workflows/drafts/` to `workflows/`.
- Workflow registration.
- Persisted steward approval records.
- Workflow catalog/store-backed proposal state.
- Workflow-declared steward configuration.
- Enterprise admin/steward controls.
- RBAC/IdP integration.
- CLI rendering beyond bounded preview output.
- Schemas and examples.
- Hosted/distributed runtime behavior.
- Write-capable adapters.

## 15. Open Questions

- Should steward review be a pure CLI helper first, or should it introduce a small typed model in `workflow-core` before CLI output?
- Should approval reasons be optional for local users or required from the first implementation?
- Should content hash freshness use canonical YAML hash only, or also include path and workflow id?
- Should steward review validate local check requirements declaratively, or defer all check execution evidence until a later phase?
- Should steward-review output be consumable by future report artifacts?
- How should local single-user permissive profiles map to enterprise steward requirements?

## 16. Final Recommendation

The next implementation should be steward-review helper/model only, in-memory and non-mutating.

It should not promote workflows, move files, register active specs, persist approvals, create runtime state, run commands, call providers, add schemas, add examples, enable writes, or change release posture.
