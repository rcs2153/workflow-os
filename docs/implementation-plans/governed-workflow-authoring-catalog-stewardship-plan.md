# Governed Workflow Authoring Catalog And Stewardship Plan

Status: Core model implemented in
[Governed Workflow Authoring Catalog And Stewardship Core Model Report](../concepts/GOVERNED_WORKFLOW_AUTHORING_CATALOG_STEWARDSHIP_CORE_MODEL_REPORT.md).
The core model was accepted in
[Governed Workflow Authoring Catalog And Stewardship Core Model Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_CATALOG_STEWARDSHIP_CORE_MODEL_REVIEW.md),
and local catalog persistence is planned in
[Workflow Catalog Persistence And Stewardship Integration Plan](workflow-catalog-persistence-plan.md).
The first local catalog store helper is implemented and accepted in
[Workflow Catalog Store Helper Review](../concepts/WORKFLOW_CATALOG_STORE_HELPER_REVIEW.md),
and workflow catalog indexing/conflict helper planning is documented in
[Workflow Catalog Indexing And Conflict Helper Plan](workflow-catalog-indexing-conflict-plan.md).
The first pure in-memory indexing/conflict helper is implemented in
[Workflow Catalog Indexing Conflict Helper Report](../concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_REPORT.md).
This follows the accepted explicit archive-command review in
[Governed Workflow Authoring Draft Archive Command Implementation Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_DRAFT_ARCHIVE_COMMAND_IMPLEMENTATION_REVIEW.md).

## 1. Executive Summary

Workflow OS can now move a reviewed inactive draft from the active draft queue
into `workflows/drafts/archive/` without deleting it. That solves local draft
hygiene, but it does not create durable knowledge about why a workflow exists,
who reviewed it, what evidence supported the decision, or how archived drafts
relate to active workflow files.

The next authoring question is catalog stewardship: how Workflow OS should
eventually record workflow identity, lifecycle, ownership, review decisions,
promotion lineage, archive actions, and conflicts without turning loader-visible
workflow files into an opaque database.

The first implementation adds model-only catalog, stewardship, and archive
record vocabulary with validation and serde. It does not implement catalog
storage, persisted approvals, runtime behavior, schemas, examples, provider
calls, deletion behavior, hosted behavior, writes, or release posture changes.

## 2. Goals

- Define a local-first workflow catalog concept for authored workflows.
- Preserve loader-visible workflow files as the current execution source of
  truth.
- Define durable stewardship records for draft review, promotion, rejection,
  supersession, and archive decisions.
- Link first-run recommendations, draft proposals, preflight results, steward
  review cards, promotion, active workflow files, and archived drafts.
- Prevent stale or reviewless promotion claims.
- Prepare for team-scale workflow ownership and lifecycle governance.
- Keep catalog records reference-oriented and redaction-safe.
- Preserve single-user automation posture while preparing enterprise steward
  review.
- Define a small future implementation sequence.

## 3. Non-Goals

Do not implement in this phase:

- catalog store code;
- persisted approval consumption;
- runtime workflow registration changes;
- automatic workflow generation;
- automatic promotion;
- automatic archive cleanup;
- draft deletion or abandon behavior;
- workflow schema changes;
- examples;
- provider calls;
- command execution;
- local check execution;
- hosted or distributed behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Current Boundary

Current authoring behavior is file-based and explicit:

1. `workflow-os first-run` emits review-only workflow recommendations.
2. `workflow-os first-run --recommendation <id>` explains a recommendation.
3. `workflow-os author workflow --from-recommendation <id> --dry-run` previews
   inactive authoring obligations.
4. `workflow-os author workflow --from-recommendation <id> --output
   workflows/drafts/<name>.workflow.yml` writes one inactive draft.
5. `workflow-os author workflow preflight --draft ...` checks promotability.
6. `workflow-os author workflow steward-review --draft ...` prints a bounded
   review card and decision result.
7. `workflow-os author workflow promote --draft ...` writes one active workflow
   file and preserves the draft.
8. `workflow-os author workflow draft-status --draft ...` inspects whether a
   draft is an active candidate, promoted-preserved, or superseded by active.
9. `workflow-os author workflow archive-draft --draft ...` moves one eligible
   promoted or superseded draft into `workflows/drafts/archive/`.

Missing boundary:

- no durable catalog identity;
- no persisted steward decision record;
- no durable promotion lineage record;
- no archive metadata record;
- no catalog conflict index;
- no stable lifecycle history beyond Git/file placement;
- no team-scale ownership or escalation review state.

## 5. Source-Of-Truth Boundaries

Workflow OS should keep these boundaries explicit:

- Workflow spec file: loader-visible definition used for validation and
  execution.
- Draft workflow file: inactive proposal artifact under `workflows/drafts/`.
- Archived draft file: inactive historical proposal under
  `workflows/drafts/archive/`.
- Stewardship record: durable record that a human or delegated maintainer
  reviewed, approved, rejected, promoted, superseded, or archived a workflow
  proposal.
- Workflow catalog record: durable index of workflow identity, lifecycle,
  ownership, active file path, content hash, and stewardship references.
- Runtime run state: execution state and event log for a workflow run.
- WorkReport: governed handoff artifact for completed work.

The catalog must not replace validation. The active workflow file must remain
validated by the loader. Catalog records should cite files and hashes rather
than copying raw YAML.

## 6. Candidate Catalog Model

Future model types may include:

- `WorkflowCatalogRecord`
- `WorkflowCatalogRecordId`
- `WorkflowCatalogVersion`
- `WorkflowLifecycleStatus`
- `WorkflowCatalogIdentity`
- `WorkflowCatalogSource`
- `WorkflowCatalogLineage`
- `WorkflowCatalogOwner`
- `WorkflowCatalogConflict`
- `WorkflowStewardshipRecord`
- `WorkflowStewardshipDecision`
- `WorkflowStewardshipDecisionId`
- `WorkflowStewardshipActor`
- `WorkflowArchiveRecord`
- `WorkflowPromotionRecord`

These model types are now implemented as core vocabulary and validation only.

## 7. Required Catalog Identity Fields

A future catalog record should capture:

- catalog record id;
- workflow id;
- workflow spec path;
- workflow content hash;
- workflow schema version;
- workflow lifecycle status;
- source recommendation id, if any;
- source draft path, if any;
- archived draft path, if any;
- owner;
- escalation contact;
- authority scope;
- evidence/check/report obligation posture;
- side-effect posture;
- latest steward decision reference;
- latest promotion reference;
- latest archive reference;
- created_at;
- updated_at;
- sensitivity;
- redaction metadata.

All paths should be repository-relative and path-safe. Absolute private paths
must not be stored.

## 8. Required Stewardship Record Fields

A future stewardship record should capture:

- stewardship decision id;
- decision kind;
- workflow id;
- draft path, if relevant;
- active workflow path, if relevant;
- archive path, if relevant;
- candidate content hash;
- active content hash, if relevant;
- reviewer actor;
- decision timestamp;
- bounded reason code or bounded reason summary;
- preflight result reference;
- steward-review result reference;
- evidence reference ids, if available;
- approval reference ids, if available;
- policy decision references, if available;
- validation diagnostic references, if available;
- known limitations;
- strict non-goals;
- sensitivity;
- redaction metadata.

Decision kinds should include:

- `draft_created`
- `review_requested`
- `approved_for_promotion`
- `rejected`
- `needs_changes`
- `promoted`
- `archived`
- `superseded`

The record should store references and bounded summaries only. It must not copy
raw workflow YAML, raw source contents, command output, provider payloads, or
secret-like values.

## 9. Archive Metadata Policy

Archiving a draft should eventually create an archive metadata record, but the
current `archive-draft` command intentionally does not.

A future archive record should cite:

- original draft path;
- archive path;
- workflow id;
- draft content hash;
- matching active workflow path, if any;
- active workflow content hash, if any;
- prior draft status;
- archive actor;
- archive reason summary or code;
- timestamp;
- validation result reference;
- related promotion/stewardship decision reference.

Archive metadata must not claim that an archive action is a persisted approval
unless the approval decision is actually recorded through the approved
stewardship model.

## 10. Lifecycle And Conflict Policy

Future catalog lifecycle statuses should be bounded and explicit:

- `draft`
- `review_pending`
- `approved`
- `active`
- `superseded`
- `archived`
- `deprecated`
- `rejected`

Future conflict checks should reason about:

- duplicate workflow ids;
- overlapping purpose;
- overlapping authority scope;
- overlapping side-effect boundary;
- conflicting policy gates;
- conflicting approval posture;
- conflicting owner/escalation assignments;
- stale draft content hashes;
- archived draft re-promotion attempts;
- active workflow path mismatch.

The first catalog implementation should disclose conflicts. Blocking promotion
from catalog conflicts should be a separate reviewed implementation phase.

## 11. Privacy And Redaction

Catalog and stewardship records must not store or print:

- raw workflow YAML;
- raw draft YAML;
- source contents;
- manifest bodies;
- package script bodies;
- dependency values;
- lockfile contents;
- CI logs;
- command output;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like strings;
- unbounded reviewer reasons;
- existing agent instruction bodies.

Allowed content:

- workflow ids;
- repository-relative paths;
- content hashes;
- lifecycle status codes;
- decision codes;
- bounded summaries;
- stable references to evidence, validation diagnostics, policy decisions,
  approvals, hook disclosures, side effects, and report artifacts.

## 12. Relationship To Approvals

Current authoring commands accept reviewer and reason inputs, but they do not
create persisted approval records.

Future stewardship records may cite approval decisions, but they must not
pretend that current same-process CLI reviewer inputs are enterprise
authorization. High-assurance approval controls, separation of requester and
approver, expiry, revocation, quorum, and IdP-backed authority remain separate
future lanes.

## 13. Relationship To Evidence And Work Reports

Catalog stewardship should cite evidence and reports by stable references.

It should be able to cite:

- EvidenceReference ids;
- validation diagnostics;
- policy decisions;
- hook disclosure ids;
- side-effect discovery records;
- WorkReport ids;
- report artifact ids;
- audit event ids.

It should not create evidence implicitly or copy evidence payloads. Workflow
catalog records describe workflow lifecycle, not work execution results.

## 14. Relationship To Local And Enterprise Use

Single local users may choose low-friction automation: the agent proposes
workflows, the kernel records evidence, the user or delegated maintainer
approves the boundary, and reports standardize the handoff.

Enterprise users need central stewardship: admins or workflow owners decide
which gates, evidence, approvals, side-effect posture, and reports are required
before workflows become active.

The catalog model should support both without making enterprise RBAC claims in
v0.

## 15. Error Handling

Future implementation errors should use stable non-leaking codes.

Candidate codes:

- `workflow_catalog.record.invalid_id`
- `workflow_catalog.record.missing_workflow_id`
- `workflow_catalog.record.missing_content_hash`
- `workflow_catalog.record.unsafe_path`
- `workflow_catalog.record.duplicate_workflow_id`
- `workflow_catalog.record.stale_content_hash`
- `workflow_catalog.stewardship.missing_decision`
- `workflow_catalog.stewardship.invalid_actor`
- `workflow_catalog.stewardship.reason_rejected`
- `workflow_catalog.archive.destination_conflict`

Errors must not echo raw YAML, raw reason text, source snippets, private paths,
command output, provider payloads, parser payloads, or secret-like values.

## 16. Test Plan

Future tests should cover:

- valid minimal catalog record;
- invalid catalog record id rejected;
- invalid workflow id rejected;
- unsafe absolute paths rejected;
- traversal paths rejected;
- missing content hash rejected;
- lifecycle status vocabulary;
- valid stewardship decision record;
- invalid reviewer actor rejected;
- secret-like reason rejected without leakage;
- promotion lineage references draft and active hashes;
- archive metadata references original and archive paths;
- duplicate workflow id conflict disclosed;
- stale draft hash conflict disclosed;
- serialization round trip;
- invalid serialized records fail closed;
- Debug output redacts bounded summaries and reasons;
- no raw YAML or command output stored;
- no runtime state created;
- existing authoring command tests still pass;
- docs check passes.

## 17. Proposed Implementation Sequence

Recommended future phases:

1. Catalog and stewardship core model only.
2. Local catalog persistence planning.
3. Local catalog store helper and store tests only. Completed as a
   file-backed helper with no command integration and accepted in
   [Workflow Catalog Store Helper Review](../concepts/WORKFLOW_CATALOG_STORE_HELPER_REVIEW.md).
4. Workflow catalog indexing/conflict helper planning. Documented in
   [Workflow Catalog Indexing And Conflict Helper Plan](workflow-catalog-indexing-conflict-plan.md).
5. Pure in-memory catalog indexing helper from existing workflow/draft files
   and catalog records. Completed in the
   [Workflow Catalog Indexing Conflict Helper Report](../concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_REPORT.md).
6. Pure validation tests for lifecycle, lineage, path, and redaction behavior.
7. Persisted stewardship decision write plan.
8. Promotion command integration with persisted catalog update.
9. Promotion command integration with persisted stewardship checks.
10. Archive command integration with archive metadata record.
11. Catalog conflict detection in steward review/preflight.
12. Review before any schema, hosted, enterprise, or write-capable behavior.

The next phase should review the pure in-memory catalog indexing/conflict
helper before any command integration.

## 18. Open Questions

- Should catalog records live under `.workflow-os/catalog/` or another local
  state surface?
- Should catalog records be committed to Git or treated as local state?
- Should catalog records be derived from Git history, Workflow OS state, or
  explicit local files?
- How should catalog records relate to future hosted/collaborative backends?
- Should archive metadata be written by `archive-draft` immediately once the
  model exists?
- Should promotion require a persisted stewardship decision before writing the
  active workflow file?
- How should single-user delegated maintainer approval differ from enterprise
  steward approval?
- What is the smallest useful conflict taxonomy for blocking promotion?
- How should catalog records cite future reasoning lineage or typed handoffs?

## 19. Final Recommendation

The local workflow catalog store helper is implemented and accepted according to
[Workflow Catalog Persistence And Stewardship Integration Plan](workflow-catalog-persistence-plan.md):
model-backed, file-backed, and still without command integration.

Workflow catalog indexing and conflict helper planning is documented in
[Workflow Catalog Indexing And Conflict Helper Plan](workflow-catalog-indexing-conflict-plan.md).
The first helper is implemented as core model/helper code only.

The next phase should review the pure in-memory indexing/conflict helper before
any persisted stewardship command wiring or promotion/archive integration.

It must still not build runtime workflow registration, automatic generation,
automatic promotion, automatic archive cleanup, deletion, workflow schema
changes, examples, providers, writes, hosted behavior, RBAC, IdP integration, or
release posture changes.
