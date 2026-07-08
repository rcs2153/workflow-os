# Workflow Catalog Repair Proposal Review And Approval Plan

## 1. Executive Summary

The workflow catalog repair dry-run CLI now exposes bounded repair proposals
without mutation:

```text
workflow-os author workflow catalog-repair --dry-run
```

The next safety question is not how to apply those proposals. The next safety
question is how a maintainer reviews, approves, rejects, defers, records, and
cites a repair proposal before any future apply mode exists.

This plan defines a conservative repair proposal review and approval boundary.
It keeps active workflow files as the execution source of truth, keeps catalog
records as local lifecycle sidecars, and treats repair proposals as
review-required recommendations.

The first in-memory review model/helper slice is implemented in
[Workflow Catalog Repair Proposal Review Helper Report](../concepts/WORKFLOW_CATALOG_REPAIR_PROPOSAL_REVIEW_HELPER_REPORT.md).
It records bounded maintainer review decisions against typed repair proposals
and supports stale proposal identity checks without persistence or mutation.
This plan does not implement apply mode, automatic repair, record deletion,
record overwrite, workflow movement, runtime state, provider calls, schemas,
examples, hosted behavior, or release posture changes.

## 2. Goals

- Create a durable, deterministic review boundary for repair proposals.
- Preserve the dry-run command as non-mutating proposal output.
- Require explicit maintainer or steward decisions before any future apply
  phase can consume a repair proposal.
- Capture reviewer, reason, proposal id, decision outcome, timestamp,
  sensitivity, and redaction posture.
- Support citations to the proposal, catalog-status conflict, workflow id,
  catalog record id, stewardship decision id, archive record id, approval
  reference, policy decision event, and future WorkReport references where
  available.
- Keep review summaries bounded and redaction-safe.
- Make proposal review useful for later apply-mode planning without
  authorizing apply mode.
- Preserve deterministic ordering and stable decision identifiers.
- Fail closed on invalid or stale review inputs.

## 3. Non-Goals

Do not implement in this phase:

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

## 4. Current Boundary

Implemented catalog repair surfaces are currently proposal-only:

- `catalog-status` reports inventory and conflicts.
- `propose_workflow_catalog_repairs` maps existing conflicts into bounded
  repair proposals.
- `catalog-repair --dry-run` exposes those proposals in human and preview JSON
  output.

None of these surfaces can create catalog records, delete records, overwrite
records, move workflows, append runtime events, or call providers.

The review/approval boundary planned here should sit between dry-run proposal
output and any future apply design:

```text
catalog-status -> catalog-repair --dry-run -> review/approval record -> future apply planning
```

## 5. Review Artifact Concept

The first implementation adds the smallest local model needed to represent a
reviewed repair proposal decision:

- `WorkflowCatalogRepairProposalReview`;
- `WorkflowCatalogRepairProposalReviewId`;
- `WorkflowCatalogRepairProposalDecisionKind`;
- `WorkflowCatalogRepairProposalReviewInput`.

The review artifact should record:

- review id;
- proposal id;
- proposal action kind;
- proposal conflict kind;
- workflow id when available;
- source category;
- bounded source reference;
- reviewer actor;
- bounded reason;
- decision kind;
- reviewed timestamp;
- optional approval references;
- optional policy decision references;
- optional evidence references;
- optional WorkReport or report artifact references when available later;
- sensitivity;
- redaction metadata.

The review artifact must not copy raw workflow YAML, raw catalog record payloads,
source file contents, command output, provider payloads, parser payloads, CI
logs, environment values, credentials, tokens, or secret-like values.

## 6. Decision Kinds

The first review model should support bounded decision kinds:

- `ApprovedForFutureApplyPlanning`;
- `Rejected`;
- `Deferred`;
- `RequiresManualCatalogReview`;
- `RequiresManualWorkflowReview`;
- `RequiresNewDryRun`.

`ApprovedForFutureApplyPlanning` is intentionally not `Applied`.

It means a maintainer agrees that this proposal may be considered by a future
apply-mode planner. It does not permit mutation by itself.

## 7. Review Input Rules

Review must be constructed from explicit inputs:

- repair proposal id;
- proposal action kind;
- proposal conflict kind;
- workflow id if available;
- source category;
- bounded source reference;
- reviewer actor;
- bounded reviewer reason;
- decision kind;
- reviewed timestamp;
- sensitivity;
- redaction metadata;
- optional stable references.

The review helper must not read hidden global state, scan arbitrary files, infer
reviewer identity from git config, or inspect raw proposal output text. It
should consume typed proposal values or explicitly supplied bounded fields.

## 8. Staleness And Re-Review

Repair proposals are derived from catalog status at a point in time. A review
record should therefore capture enough proposal identity to detect stale reuse:

- proposal id;
- conflict kind;
- action kind;
- source category;
- source reference;
- workflow id when available.

Future apply-mode planning must require a fresh dry-run proposal that still
matches the reviewed proposal. If the catalog-status conflict disappears,
changes kind, or points at a different source reference, the review should not
be reusable without explicit re-review.

## 9. Approval And Policy Relationship

Repair proposal review is a maintainer/steward decision boundary. It may cite
approval references and policy decision events, but it must not implement a
general enterprise approval system in this phase.

Rules:

- optional approval references may be attached when an external or existing
  Workflow OS approval checkpoint was used;
- optional policy decision references may be attached when a policy gate was
  evaluated;
- missing approval or policy references must be explicit and safe;
- review construction must not fabricate approval ids or policy event ids;
- approval references must not imply apply permission unless a later apply
  phase explicitly validates them.

## 10. CLI Surface Recommendation

The first implementation should prefer a model/helper before CLI mutation.

If a CLI preview is later added, it should remain non-mutating and explicitly
named as review recording, not repair application. Candidate shape:

```text
workflow-os author workflow catalog-repair review \
  --proposal-id <id> \
  --decision <approved-for-future-apply-planning|rejected|deferred|requires-manual-review> \
  --reviewer <actor> \
  --reason <bounded-reason> \
  --dry-run
```

Any actual persistence of a review record should be separately planned and
reviewed. This plan does not authorize a CLI write path.

## 11. Persistence Posture

The first implementation should be model/helper-only or preview-only.

Persisted review records are useful, but they should be a separate opt-in phase
because persistence introduces:

- local sidecar file naming;
- overwrite/refusal semantics;
- duplicate review handling;
- stale review handling;
- store health summaries;
- future apply-mode lookup semantics.

The first implementation constructs repair proposal review records in memory
and tests validation, redaction, serialization, and stale proposal identity.
Persisted review records remain deferred until a separate opt-in persistence
phase is planned and reviewed.

## 12. Future Apply Relationship

Future apply mode must require more than a review record.

Before applying anything, a future phase must validate:

- the active workflow or archived draft still exists where expected;
- the current conflict still matches the reviewed proposal;
- the review decision kind permits future apply planning;
- the requested apply action is among the reviewed action kinds;
- source reference, workflow id, hash posture, and catalog root are still safe;
- no deletion or overwrite is implied unless separately designed;
- the operation can fail safely without corrupting catalog state.

This plan does not authorize apply mode.

## 13. Error Handling

Validation and construction errors must use stable, non-leaking codes.

Candidate future codes:

- `workflow_catalog.repair_review.invalid_proposal_id`;
- `workflow_catalog.repair_review.invalid_decision`;
- `workflow_catalog.repair_review.invalid_reviewer`;
- `workflow_catalog.repair_review.invalid_reason`;
- `workflow_catalog.repair_review.invalid_reference`;
- `workflow_catalog.repair_review.stale_proposal`;
- `workflow_catalog.repair_review.unsupported_apply_posture`;

Errors must not echo raw reasons, raw source references outside bounded
repository-relative metadata, raw catalog payloads, YAML snippets, paths with
secret-like values, tokens, credentials, provider payloads, command output, or
environment values.

## 14. Privacy And Redaction

Repair review records may reveal governance gaps and ownership posture. They
should default conservatively.

Rules:

- use validated constructors;
- bound reviewer reasons;
- reject or redact secret-like reviewer reasons;
- treat source references as bounded metadata, not raw paths to disclose
  freely;
- require redaction metadata;
- keep Debug output redaction-safe;
- keep serialization safe;
- keep deserialization fail-closed through constructors;
- do not include raw proposal JSON output as an embedded payload.

## 15. Test Plan

Future implementation tests should cover:

- valid review for a missing-catalog-record proposal;
- valid rejection for a proposal;
- valid deferred decision for a proposal;
- invalid proposal id rejected;
- invalid reviewer rejected;
- empty or unbounded reason rejected;
- secret-like reason rejected without leakage;
- unsupported decision kind rejected;
- review records preserve proposal action/conflict/source identity;
- optional approval references are bounded and valid;
- optional policy decision references are bounded and valid;
- evidence references are cited by stable id only;
- Debug output does not leak reviewer reason or sensitive source details;
- serialization does not leak forbidden payload fields;
- invalid serialized review fails closed;
- stale proposal detection is representable, even if apply validation remains
  deferred;
- no files are written by model/helper tests;
- no catalog records are created, updated, overwritten, or deleted;
- existing catalog repair proposal, catalog status, promotion, archive, and
  stewardship tests still pass;
- docs check passes.

## 16. Proposed Implementation Sequence

1. Review this plan.
2. Add a core model/helper for in-memory repair proposal review records.
   Implemented in
   [Workflow Catalog Repair Proposal Review Helper Report](../concepts/WORKFLOW_CATALOG_REPAIR_PROPOSAL_REVIEW_HELPER_REPORT.md).
3. Add focused validation, serde, and redaction tests.
4. Review the core model/helper.
5. Plan optional local persistence for review records.
6. Only after persistence review, plan whether a CLI review-record write path
   should exist.
7. Only after review-record persistence is accepted, plan a narrow future apply
   mode for the safest missing-record creation cases.

## 17. Deferred Work

- repair apply mode;
- persisted review sidecars;
- review CLI write path;
- automatic repair;
- catalog cleanup;
- deletion;
- overwrite/update apply mode;
- rollback;
- runtime event or audit append;
- WorkReport artifact generation;
- schema exposure;
- examples;
- hosted/team catalog store behavior;
- RBAC, IdP, notifications, or admin UI;
- provider writes;
- release posture changes.

## 18. Open Questions

- Should repair proposal reviews share the existing stewardship record store or
  use a separate repair-review namespace?
- Should a review decision expire after catalog status changes?
- Should review records include a hash of the proposal fields or only typed
  fields?
- Should approval references be required for any future apply mode, or only for
  high-risk actions?
- Should rejected repair proposals be persisted to prevent repeated noisy
  recommendations?
- Should `catalog-repair --dry-run --json` eventually emit a review command
  template, or would that imply too much automation?
- What is the smallest review record that gives future apply mode enough safety
  without becoming a second workflow system?

## 19. Final Recommendation

Proceed next to maintainer review of the in-memory repair proposal review
model/helper implementation.

Do not implement apply mode, automatic repair, persisted review sidecars, CLI
write behavior, deletion, overwrite, workflow movement, runtime state, schemas,
examples, hosted behavior, provider calls, write-capable adapters, or release
posture changes.
