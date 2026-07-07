# Governed Workflow Authoring Steward Review Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implemented steward-review helper is appropriately narrow, in-memory, explicit-input, redaction-safe, and non-mutating. It satisfies the accepted helper/model-only boundary and is suitable as the foundation for a future bounded CLI steward-review preview surface.

## 2. Scope Verification

The phase stayed within the approved helper-only scope.

Implemented scope:

- core steward-review helper and model vocabulary;
- explicit review input model;
- bounded review card result;
- future-promotion authorization vocabulary;
- non-mutation boundary record;
- deterministic validation;
- redaction-safe Debug behavior;
- serde support through model derives;
- focused tests;
- roadmap and plan documentation updates;
- implementation report.

No accidental implementation was found for:

- steward-review CLI command;
- active workflow promotion;
- workflow registration;
- file movement from `workflows/drafts/` to `workflows/`;
- persisted steward approvals;
- runtime state creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- workflow-declared steward configuration;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- RBAC, IdP, admin UI, paging, or notifications;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 3. Model And API Assessment

The model is domain-appropriate and minimal for this phase.

The implementation adds:

- `WorkflowDraftPromotionPreflightStatus`;
- `WorkflowDraftStewardReviewDecision`;
- `WorkflowDraftStewardReviewAuthorization`;
- `WorkflowDraftStewardReviewInput`;
- `WorkflowDraftStewardReviewCard`;
- `WorkflowDraftStewardReviewBoundary`;
- `WorkflowDraftStewardReviewResult`;
- `review_workflow_draft_for_promotion`.

The API accepts explicit caller-supplied review context rather than reading repository state, runtime state, global configuration, or hidden process state. That is the right boundary for the first slice because it lets future CLI/runtime surfaces compose the helper without smuggling in file movement or active registration behavior.

The helper returns a structured result instead of printing CLI output or mutating state. The public exports from `workflow-core` are consistent with the existing model/helper pattern used elsewhere in the repository.

## 4. Steward Review Boundary Assessment

The helper correctly models steward review as a decision boundary between an inactive draft and a future separately implemented promotion step.

Approval means:

- the exact unchanged draft may proceed to a later promotion step;
- the preflight status and content hash were supplied;
- the reviewer supplied a bounded decision and reason;
- the helper returned `AuthorizedForPromotion` only for `ApprovedForPromotion`.

Approval does not mean:

- the draft was promoted;
- files were moved;
- a workflow was registered;
- commands were run;
- provider calls were made;
- runtime state was created;
- approval state was persisted;
- future edits to the draft are approved.

This preserves the plan's separation between review, promotion, and runtime execution.

## 5. Validation Assessment

Validation is deterministic and fails closed for the core reviewed conditions.

Verified behavior:

- draft path must be relative, under `workflows/drafts/`, and end in `.workflow.yml`;
- candidate workflow id must not remain in the `draft/` namespace;
- current draft hash must match the hash that preflight evaluated;
- blocked preflight status fails closed;
- non-empty preflight blocker list fails closed;
- active workflow conflict fails closed;
- review summaries and approval reason must be non-empty, bounded, and non-secret-like;
- preflight blocker and warning codes are bounded, validated, and duplicate-checked;
- stable error codes are used;
- raw blocker payloads and secret-like input values are not echoed in tested errors.

The helper does not parse draft files or run preflight itself, which is correct for this phase. Callers remain responsible for supplying a fresh preflight result.

## 6. Decision And Authorization Assessment

Decision semantics match the plan.

- `ApprovedForPromotion` maps to `AuthorizedForPromotion`;
- `Denied` maps to `NotAuthorized`;
- `NeedsChanges` maps to `NotAuthorized`;
- `Deferred` maps to `NotAuthorized`.

The authorization is intentionally narrow: it authorizes only a future separately implemented promotion step for the exact unchanged draft. The helper does not create durable approval authority and does not itself become a promotion mechanism.

## 7. Non-Mutation Boundary Assessment

The non-mutation boundary is explicit and useful.

The result exposes false flags for:

- `files_written`;
- `workflow_registered`;
- `workflow_promoted`;
- `approval_persisted`;
- `runtime_state_created`;
- `commands_executed`;
- `providers_called`.

This is a good reviewable contract for future phases because it lets tests and callers assert that steward review remains pure until a separately approved promotion implementation exists.

## 8. Privacy And Redaction Assessment

The implementation is redaction-safe for the current helper boundary.

Verified posture:

- review summary text is bounded;
- secret-like summary and approval-reason values are rejected;
- Debug redacts caller-supplied review summaries and approval reasons;
- errors use stable codes and generic messages;
- raw draft YAML is not read or copied;
- raw source contents are not read or copied;
- raw package scripts are not copied;
- raw CI logs are not copied;
- command output is not copied;
- provider payloads are not copied;
- parser payloads are not copied;
- environment variable values, credentials, authorization headers, private keys, and token-like strings are not copied.

The helper allows bounded relative draft paths and stable content hashes, which is appropriate for review identity without exposing raw draft content.

## 9. Test Quality Assessment

The focused tests cover the important first-slice behavior:

- approved preflight-passing draft authorizes future promotion without mutation;
- denied, needs-changes, and deferred decisions do not authorize promotion;
- preflight blockers fail closed without leaking blocker payloads;
- stale preflight hash fails closed;
- active workflow conflict fails closed;
- unsafe or secret-like inputs are rejected without leakage;
- Debug output redacts bounded review text.

The tests also follow repository style by avoiding `expect`/`panic` shortcuts under clippy's strict settings.

No blocker-level test gaps were found.

Non-blocking gaps to consider later:

- add direct assertions for duplicate warning/blocker codes;
- add direct assertions for too-long bounded summary text;
- add direct tests for `draft/` namespace rejection;
- add serde round-trip tests if the helper result becomes a persisted or CLI JSON-facing surface;
- add accessors for policy, evidence/report, side-effect, and next-action card fields when a caller needs them.

## 10. Documentation Review

Documentation is honest about the implemented and unimplemented boundaries.

Docs state that the first pure in-memory steward-review helper is implemented and that the following remain unimplemented:

- steward-review promotion command;
- active workflow registration;
- file movement;
- runtime state;
- schemas;
- examples;
- hosted behavior;
- writes;
- release posture changes.

The implementation report includes scope completed, explicit non-scope, API summary, validation boundary, decision semantics, non-mutation boundary, privacy posture, tests, dogfood summary, validation commands, known limitations, and recommended next phase.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Add a bounded CLI steward-review preview plan before active promotion.
- Add direct tests for duplicate code lists, too-long review text, and `draft/` namespace rejection.
- Add serde round-trip tests if the result becomes JSON-facing.
- Add card accessors for policy, evidence/report, side-effect, and next-action fields when required by the CLI or promotion path.
- Keep approval-presentation proof as a separate P0 hardening lane before relying on steward decisions for higher-risk promotion surfaces.

## 13. Recommended Next Phase

Recommended next phase: governed workflow authoring steward-review CLI preview planning.

The helper is accepted, but active promotion should not be implemented yet. The next safest product step is a bounded preview surface that can present the review card and decision result to a maintainer without moving files, registering workflows, persisting approval state, creating runtime state, executing commands, calling providers, adding schemas, adding examples, enabling writes, or changing release posture.

## 14. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783434010043711000-2 --phase review`: passed.

Governed review run:

- Workflow: `dg/review`.
- Run ID: `run-1783434010043711000-2`.
- Approval ID: `approval/run-1783434010043711000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Phase-close: completed with 39 events, 1 approval, 0 retries, and 0 escalations.
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`.
