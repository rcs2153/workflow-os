# Workflow Catalog Repair Proposal Review Helper Review

## 1. Executive Verdict

Phase accepted; proceed to repair review persistence planning.

The phase adds the intended in-memory review model/helper for workflow catalog
repair proposals. It records bounded maintainer decisions against typed
non-mutating repair proposals, preserves proposal identity for stale-review
checks, and does not implement persistence, CLI write behavior, repair apply
mode, automatic repair, catalog mutation, runtime registration, schemas,
examples, hosted behavior, provider calls, writes, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved model/helper-only scope.

No accidental implementation was found for:

- repair apply mode;
- automatic catalog repair;
- persisted review sidecars;
- CLI review write behavior;
- catalog record creation, update, overwrite, deletion, or cleanup;
- active workflow rewrite;
- draft or archive movement;
- runtime workflow registration;
- runtime state creation;
- event or audit append;
- report artifact generation;
- schema exposure;
- examples;
- provider calls;
- write-capable adapters;
- hosted or team catalog backend behavior;
- release posture changes.

## 3. Model Assessment

The model is appropriately minimal for the reviewed boundary.

Implemented concepts:

- `WorkflowCatalogRepairProposalReviewId`;
- `WorkflowCatalogRepairProposalDecisionKind`;
- `WorkflowCatalogRepairProposalReviewInput`;
- `WorkflowCatalogRepairProposalReview`;
- `review_workflow_catalog_repair_proposal`;
- `validate_workflow_catalog_repair_proposal_review_matches`.

The review record captures the necessary proposal identity fields, reviewer
actor, bounded reason, decision kind, timestamp, optional stable references,
sensitivity, and redaction metadata. It does not read hidden state or derive
authority from ambient repository configuration.

## 4. Decision Boundary Assessment

The supported decision kinds are safe and explicit:

- `ApprovedForFutureApplyPlanning`;
- `Rejected`;
- `Deferred`;
- `RequiresManualCatalogReview`;
- `RequiresManualWorkflowReview`;
- `RequiresNewDryRun`.

`ApprovedForFutureApplyPlanning` remains correctly scoped. It does not apply a
repair and does not authorize mutation by itself. It only records that a future
apply planner may consider the reviewed proposal after separate freshness,
policy, and mutation checks.

## 5. Staleness Assessment

The helper preserves enough proposal identity for future stale-review
detection:

- proposal id;
- proposal action kind;
- proposal conflict kind;
- proposal conflict source;
- workflow id when available;
- source reference.

`validate_workflow_catalog_repair_proposal_review_matches` fails closed with
the stable code `workflow_catalog_repair.review.stale_proposal` when a review no
longer matches a fresh proposal identity. The error does not leak renamed paths
or source-reference details.

## 6. Citation And Reference Assessment

The model supports optional stable references for:

- approval references;
- policy decision event references;
- evidence references;
- validation references;
- WorkReport references.

These references are citations only. The helper does not fabricate ids, create
approval or policy authority, create evidence, or validate external approval
semantics. That is appropriate for this phase.

## 7. Validation Assessment

Validation is deterministic and fail-closed for the model boundary.

Verified behavior:

- review ids use the repository identifier validation path;
- reviewer actors are validated before review construction;
- reviewer reasons are bounded and secret-like values are rejected;
- optional reference counts are bounded;
- redaction metadata is validated;
- deserialization re-runs reason, source-reference, reference-count, and
  redaction validation;
- stale proposal checks return a stable non-leaking validation error.

No validation blocker was found.

## 8. Privacy And Redaction Assessment

The implementation does not copy raw workflow YAML, raw catalog records, source
file contents, command output, provider payloads, parser payloads, CI logs,
environment values, credentials, tokens, or secret-like values.

Debug output redacts reviewer reason and source reference and reports reference
counts instead of expanding reference payloads. Redaction metadata is formatted
through the existing redacted debug wrapper.

Serialization stores bounded review fields, including bounded reason and source
reference, and deserialization fails closed when secret-like values are present.
That is acceptable for this in-memory model and consistent with the current
catalog-sidecar posture, provided future persistence keeps treating the record
as sensitive governance metadata.

## 9. Test Quality Assessment

Focused tests cover:

- valid approval-for-future-apply-planning review;
- rejected, deferred, manual-catalog-review, manual-workflow-review, and
  requires-new-dry-run decisions;
- invalid review id handling;
- invalid reviewer handling through `ActorId` validation;
- secret-like reason rejection without leakage;
- optional approval, policy, evidence, validation, and WorkReport references;
- serde round trip;
- invalid serialized review fail-closed behavior;
- Debug non-leakage for reason and source reference;
- stale proposal mismatch detection.

The tests are behavior-oriented and protect the approved phase boundary. They
do not test persistence, CLI write behavior, or apply behavior because those
remain explicitly out of scope.

## 10. Documentation Review

Documentation now states:

- the in-memory repair proposal review helper is implemented;
- review decisions are bounded maintainer decisions;
- approved-for-future-apply-planning is not apply permission;
- stale proposal identity checks are implemented;
- repair review persistence is not implemented;
- CLI review write behavior is not implemented;
- automatic repair is not implemented;
- apply mode is not implemented;
- deletion, overwrite, cleanup, runtime registration, schemas, examples,
  hosted behavior, providers, writes, and release posture changes remain
  deferred.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- When persistence is planned, treat serialized review records as sensitive
  governance metadata because bounded reasons and source references may still
  reveal repository posture.
- Future apply planning must require a fresh matching dry-run proposal and must
  not rely on a review record alone.
- A later CLI review-recording surface should preserve the model/helper
  boundary and remain opt-in, explicit, and non-mutating until separately
  approved.

## 13. Recommended Next Phase

Recommended next phase: repair review persistence planning.

Why: the in-memory review helper is accepted and the next useful safety
question is where, whether, and how review records should be persisted as local
catalog sidecars. That must be planned before any CLI review write behavior or
future apply-mode design.

## 14. Validation

Review-phase validation:

```text
npm run check:docs
```

Result: passed.

Focused/full implementation validation from the reviewed phase:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Result: passed.

## 15. Governed Phase Metadata

- dogfood workflow id: `dg/review`
- run id: `run-1783548174518256000-2`
- approval id:
  `approval/run-1783548174518256000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- event summary: 39 events; 1 approval; 0 retries; 0 escalations
- event kinds:
  `ApprovalGranted:1, ApprovalRequested:1, PolicyDecisionRecorded:8,
  RunCompleted:1, RunCreated:1, RunResumed:1, RunStarted:1,
  RunValidated:1, SkillInvocationRequested:6, SkillInvocationStarted:6,
  SkillInvocationSucceeded:6, StepScheduled:6`
- out-of-kernel work: documentation review, documentation edit, validation,
  git, PR, and merge operations were executed by Codex/human execution layer;
  the kernel coordinated governance only
