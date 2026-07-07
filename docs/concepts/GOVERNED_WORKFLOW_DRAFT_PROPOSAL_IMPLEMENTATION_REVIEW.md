# Governed Workflow Draft Proposal Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds an internal inactive draft proposal helper for first-run recommendations and surfaces bounded proposal obligations in `workflow-os first-run --recommendation <id>` human and preview JSON output. It remains helper-only and does not add workflow file generation, workflow registration, promotion, command execution, provider calls, runtime state creation, schemas, examples, hosted behavior, write-capable adapters, or release posture changes.

## 2. Scope Verification

The phase stayed within approved scope.

Confirmed in scope:

- internal `GovernedWorkflowDraftProposal` helper/model;
- recommendation-id validation for the helper boundary;
- inactive proposal fields in recommendation detail output;
- inactive proposal fields in preview JSON;
- focused unit tests;
- focused CLI tests;
- docs, roadmap, and phase report updates.

No accidental implementation was found for:

- workflow file writes;
- active workflow generation;
- workflow registration or promotion;
- local check registration;
- local check execution;
- command execution;
- provider calls;
- runtime state creation;
- catalog storage;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 3. Model And Helper Assessment

The helper is appropriately small and located at the existing first-run recommendation boundary where the source recommendation data currently lives.

The model captures the useful first slice:

- source recommendation id;
- inactive status;
- draft lifecycle posture;
- proposal kind;
- proposed purpose code;
- required authoring decisions;
- validation expectations;
- missing required fields;
- explicit non-goals;
- privacy boundary.

This gives agents and maintainers a more concrete authoring checklist without pretending an active workflow exists.

## 4. User-Facing Output Assessment

The `first-run --recommendation <id>` detail output now includes inactive proposal obligations. This is a good extension of the existing detail view because it improves actionability without introducing a separate authoring command.

The output remains explicit that proposals are inactive and non-mutating. It lists non-goals such as no file write, no workflow registration, no command execution, no provider call, and no runtime state creation.

## 5. JSON Assessment

Preview JSON includes a nested `draft_proposal` object with bounded fields.

The shape is suitable for preview use and does not introduce workflow schema exposure or persistence. It should remain preview-scoped until compatibility expectations are deliberately set.

## 6. Validation And Error Handling Assessment

The helper validates recommendation ids before creating a proposal.

Secret-like or unsafe recommendation ids fail with `cli.workflow_authoring.unsafe_payload_rejected`, and tests confirm the supplied secret-like id is not echoed.

The implementation does not add a public path for arbitrary proposal input, which keeps the current validation surface small.

## 7. Privacy And Redaction Assessment

The helper uses static Workflow OS vocabulary and already-bounded recommendation data. It does not copy raw repository payloads.

Review confirmed the implementation does not copy:

- source contents;
- manifest bodies;
- package script command bodies;
- dependency values;
- CI logs;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, or token-like strings.

## 8. Behavior Preservation

Default `workflow-os first-run` behavior remains unchanged.

`workflow-os first-run --verbose` remains the full posture matrix.

`workflow-os first-run --recommendation <id>` still renders one selected recommendation and creates no runtime state. The new proposal output is explanatory only.

## 9. Test Quality Assessment

Tests cover:

- inactive draft proposal shape;
- required authoring decisions;
- validation expectations;
- missing required fields;
- explicit non-goals;
- secret-like recommendation id rejection without leakage;
- side-effect proposal non-write posture;
- report/handoff closure obligations;
- human recommendation detail proposal fields;
- preview JSON proposal fields;
- no runtime state creation in recommendation detail tests.

No blocker-level test gaps were found.

Non-blocking test follow-ups:

- Add structural JSON parsing if the preview JSON shape begins to stabilize.
- Add explicit tests for each ecosystem recommendation kind if proposal text becomes ecosystem-specific.

## 10. Documentation Review

Docs correctly say:

- draft proposal output is helper-only and inactive;
- no workflow files are written;
- no workflows are registered or promoted;
- no commands are executed;
- no providers are called;
- no runtime state is created;
- schemas, examples, hosted behavior, writes, and release posture changes remain unimplemented.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Keep the proposal model CLI-internal until a reviewed authoring API boundary is planned.
- Avoid treating preview JSON as stable until compatibility rules are defined.
- Plan a separate authoring CLI surface before any file-writing path.
- Plan conflict checks before draft files can be promoted or activated.

## 13. Recommended Next Phase

Recommended next phase: governed workflow authoring CLI/dry-run planning.

The helper now gives Workflow OS an inactive proposal model. The next question is how, if at all, users should request a preview/dry-run authoring surface from the CLI without writing files, registering workflows, executing commands, or implying active governance.

That next phase should remain planning-only.

## 14. Governed Dogfood Summary

- Workflow: `dg/review`.
- Run ID: `run-1783397186527330000-2`.
- Approval ID: `approval/run-1783397186527330000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations; terminal status `Completed`.
- Scope approved: create the maintainer review document for the governed workflow draft proposal helper implementation.
- Out-of-kernel work disclosed: file editing, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 15. Validation Commands Run

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783397186527330000-2 --phase review`: passed.
