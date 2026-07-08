# Workflow Catalog Repair Proposal Review And Approval Plan Report

## 1. Executive Summary

This phase created a planning document for the review and approval boundary
that must sit between workflow catalog repair dry-run proposals and any future
repair apply mode.

The plan keeps repair proposals non-mutating and defines how future maintainers
or stewards should approve, reject, defer, cite, and record proposal decisions
before any catalog repair can be considered for application.

## 2. Scope Completed

- Added
  [Workflow Catalog Repair Proposal Review And Approval Plan](../implementation-plans/workflow-catalog-repair-proposal-review-approval-plan.md).
- Defined a future repair proposal review artifact concept.
- Defined candidate decision kinds, including
  `ApprovedForFutureApplyPlanning`, `Rejected`, `Deferred`,
  `RequiresManualCatalogReview`, `RequiresManualWorkflowReview`, and
  `RequiresNewDryRun`.
- Defined explicit input rules, staleness handling, citation posture, approval
  and policy relationships, validation/error posture, privacy requirements,
  and future test coverage.
- Updated roadmap and workflow catalog persistence planning pointers.
- Cleaned stale repair/recovery plan wording that implied the dry-run CLI was
  still only a candidate surface.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- repair apply mode;
- automatic catalog repair;
- record creation;
- record update or overwrite;
- record deletion;
- catalog cleanup;
- active workflow rewrites;
- draft or archive movement;
- runtime workflow registration;
- runtime state creation;
- event or audit append;
- report artifact generation;
- workflow schema changes;
- examples;
- hosted or team catalog backend behavior;
- provider calls;
- local check or command execution;
- write-capable adapters;
- release posture changes.

## 4. Planning Boundary Summary

The planned boundary is:

```text
catalog-status -> catalog-repair --dry-run -> review/approval record -> future apply planning
```

The review decision must not be treated as mutation permission by itself. A
future apply phase must still validate that the current catalog-status conflict
matches the reviewed proposal and that the action remains safe.

## 5. Privacy And Redaction Summary

The plan requires review records to store bounded identifiers, decision
metadata, and stable references only. It forbids raw workflow YAML, catalog
record payloads, source contents, command output, provider payloads, parser
payloads, CI logs, environment values, credentials, tokens, and secret-like
review reasons.

Debug, serialization, deserialization, and validation errors must remain
redaction-safe.

## 6. Governed Phase Summary

- dogfood workflow id: `dg/d`
- run id: `run-1783545031446057000-2`
- approval id: `approval/run-1783545031446057000-2/planning-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 events; 1 approval; 0 retries; 0 escalations
- approved scope: planning document for repair proposal review/approval before
  any future apply mode
- strict non-goals: no apply mode, automatic repair, catalog writes, deletion,
  overwrite, runtime state, schemas, examples, providers, or release posture
  changes
- out-of-kernel work: documentation edits and validation commands were executed
  by Codex/human execution layer; the kernel coordinated governance only

## 7. Validation Commands

Planned validation:

```text
npm run check:docs
```

Result: passed.

## 8. Remaining Known Limitations

- No review model/helper exists yet.
- No persisted repair review sidecar exists yet.
- No CLI review-record surface exists yet.
- No future apply mode is authorized.
- Staleness checks are planned but not implemented.

## 9. Recommended Next Phase

Recommended next phase: maintainer review of repair proposal review/approval
planning.

If accepted, the next implementation phase should add only the in-memory repair
proposal review model/helper with focused validation, serde, and redaction
tests. Apply mode, persistence, CLI writes, deletion, overwrite, workflow
movement, runtime state, schemas, examples, provider calls, write-capable
adapters, and release posture changes must remain out of scope.
