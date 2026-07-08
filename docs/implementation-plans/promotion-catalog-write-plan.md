# Promotion Catalog Write Plan

## 1. Executive Summary

Workflow OS now has three relevant building blocks:

- `workflow-os author workflow promote` can explicitly promote one reviewed
  inactive draft into the active `workflows/` surface.
- `workflow-os author workflow steward-review --persist-stewardship` can write
  one validated local catalog stewardship decision record.
- `workflow-os author workflow catalog-status` can inspect active workflows,
  drafts, archived drafts, and optional local catalog records without writing.

The next implementation question is how active promotion should optionally write
a local workflow catalog record and cite persisted stewardship without turning
promotion into runtime registration or a hidden workflow database.

This plan is planning only. It does not implement promotion catalog writes,
runtime registration, archive metadata writes, schemas, examples, providers,
hosted behavior, external writes, or release posture changes.

## 2. Goals

- Add a narrow future opt-in promotion catalog write boundary.
- Preserve active workflow files as the execution source of truth.
- Let promotion cite a persisted approved stewardship decision when supplied.
- Write or update one validated `WorkflowCatalogRecord` only after active
  workflow validation succeeds.
- Keep default `author workflow promote` behavior unchanged unless an explicit
  catalog flag is supplied.
- Fail closed before active workflow mutation when catalog prerequisites are
  invalid and required.
- Surface partial-integration status clearly if active promotion succeeds but a
  later catalog sidecar write fails.
- Keep catalog records reference-oriented and redaction-safe.
- Prepare archive metadata and catalog health phases without implementing them.

## 3. Non-Goals

Do not implement in the promotion catalog write phase:

- runtime workflow registration;
- automatic workflow generation;
- automatic promotion;
- multi-draft promotion;
- archive metadata writes;
- catalog repair;
- draft deletion;
- runtime state creation;
- workflow run creation;
- local check execution;
- provider calls;
- report artifacts;
- workflow schema changes;
- examples;
- hosted or distributed catalog behavior;
- RBAC, IdP integration, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Current Boundary

Current promotion behavior is an explicit repository authoring mutation:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

It derives fresh preflight context, validates the candidate in active-placement
context, requires same-process reviewer and reason input, refuses active path
overwrite, writes exactly one active workflow file, preserves the draft, and
reloads validation after the write.

It does not persist approval records, write workflow catalog records, start
runs, create runtime state, execute commands, call providers, or write report
artifacts.

## 5. Proposed CLI Shape

Add explicit opt-in catalog persistence flags to promotion:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --persist-catalog-record \
  [--catalog-root .workflow-os/catalog] \
  [--stewardship-decision-id stewardship/<id>]
```

Dry-run should support the same flags without writing:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --persist-catalog-record \
  --stewardship-decision-id stewardship/<id> \
  --dry-run
```

Flag rules:

- `--catalog-root` is accepted only when `--persist-catalog-record` is present.
- `--stewardship-decision-id` is accepted only when
  `--persist-catalog-record` is present.
- `--persist-catalog-record` creates the catalog root if needed because the
  user explicitly requested persistence.
- Default promotion without `--persist-catalog-record` remains unchanged.

## 6. Stewardship Decision Policy

The first catalog write implementation should support two modes:

- optional citation mode: when `--stewardship-decision-id` is supplied, read and
  validate that persisted stewardship decision before promotion;
- no-citation mode: when no stewardship decision id is supplied, catalog record
  writing may still proceed but must disclose that no persisted stewardship
  decision was cited.

Strict requirement mode should be deferred until a later policy/config phase.
The CLI should not suddenly require persisted stewardship for all local single
user promotion flows.

If a stewardship decision id is supplied, promotion must verify:

- the record exists in the selected catalog root;
- the record is valid and deserializes through the model constructor;
- `workflow_id` matches the candidate workflow id;
- `draft_path` matches the draft being promoted;
- `candidate_content_hash` matches the current draft content hash;
- decision kind authorizes promotion, such as `ApprovedForPromotion`;
- rejected, deferred, needs-changes, superseded, archived, or unrelated
  decisions do not authorize citation as approval.

Any mismatch fails closed before active workflow file mutation.

## 7. Catalog Record Construction Policy

The future implementation should construct a `WorkflowCatalogRecord` through the
existing model constructor. The record should include only bounded references:

- deterministic catalog record id;
- workflow id;
- active workflow path;
- source draft path;
- archived draft path as absent;
- workflow content hash;
- source recommendation id if available from the draft metadata;
- lifecycle status `Active`;
- owner/escalation summary where available from the workflow spec;
- authority scope summary where available;
- evidence/check/report posture summary where available;
- side-effect posture summary where available;
- latest stewardship decision id when supplied and verified;
- latest promotion decision id if the promotion decision is represented by an
  existing stewardship decision id;
- conservative sensitivity and explicit redaction metadata.

The record must not store raw workflow YAML, raw source contents, command output,
provider payloads, parser payloads, package scripts, environment values,
credentials, or token-like values.

## 8. Write Timing And Atomicity

Recommended first implementation sequence:

1. Load and validate the project.
2. Validate draft path and derive current draft content hash.
3. Run current promotion preflight.
4. If catalog persistence is requested, validate catalog root and optional
   stewardship decision id before active workflow mutation.
5. Validate active-placement candidate before writing.
6. Execute the existing active workflow file write.
7. Reload and validate the project after the active file write.
8. Construct the catalog record from the validated active workflow, draft, hash,
   and optional stewardship citation.
9. Write the catalog record through `LocalWorkflowCatalogStore` with
   write-if-absent behavior.
10. Print bounded output that distinguishes active workflow promotion from
    catalog persistence.

Invalid catalog inputs must fail before active workflow mutation. If active file
promotion succeeds but catalog record write fails after post-write validation,
the command should return a stable partial-integration error that clearly says:

- the active workflow file was promoted;
- the catalog record was not written;
- no runtime state was created;
- no automatic rollback was attempted;
- the maintainer should run catalog status or retry with an appropriate recovery
  command once planned.

Automatic rollback remains deferred.

## 9. Failure Semantics

Promotion with catalog persistence must fail closed before writing the active
workflow file when:

- `--catalog-root` is unsafe or supplied without `--persist-catalog-record`;
- `--stewardship-decision-id` is invalid or supplied without
  `--persist-catalog-record`;
- the supplied stewardship decision record is missing;
- the supplied stewardship decision record is corrupt or invalid;
- the supplied stewardship decision does not match workflow id, draft path, or
  content hash;
- the supplied stewardship decision does not authorize promotion;
- a catalog record already exists for the target record id and the first
  implementation does not yet define update semantics;
- constructing the catalog record fails.

Errors must use stable codes and must not echo raw paths, raw YAML, review
reason text, serialized record payloads, parser payloads, command output,
provider payloads, or secret-like values.

## 10. Output Policy

Text and JSON output should disclose:

- mode: `author_workflow_active_promotion`;
- catalog persistence requested: true or false;
- active workflow promoted: true or dry-run false;
- catalog record written: true or false;
- catalog record id when written;
- catalog root as a bounded repository-relative path;
- stewardship decision id when cited;
- stewardship decision required: false for the first implementation;
- stewardship decision verified: true, false, or not_available;
- runtime registration: false;
- runtime state created: false;
- commands executed: false;
- providers called: false;
- report artifacts written: false;
- next action: run `workflow-os author workflow catalog-status` or
  `workflow-os validate`.

Output must not copy review reason text, raw workflow YAML, raw catalog JSON,
source contents, command output, provider payloads, environment values,
credentials, or token-like values.

## 11. Relationship To Catalog Status

After catalog write implementation, `catalog-status` should be able to read the
new record and reduce the existing `active_workflow_missing_catalog_record`
warning/blocker for that workflow.

The promotion command should not depend on catalog-status output strings.
Instead, both commands should share validated catalog-store/index helper
behavior where practical.

Strict catalog coverage remains opt-in to `catalog-status` only until a separate
enforcement phase decides whether promotion should require catalog records by
policy.

## 12. Relationship To Archive Metadata

Promotion catalog writes should not write archive records. Draft preservation
and archive semantics remain separate.

Later archive integration can cite:

- the active workflow catalog record id;
- the related stewardship decision id;
- the archive actor and reason;
- source draft path and archive path;
- content hash.

This phase should avoid coupling promotion catalog record writes to archive
cleanup.

## 13. Privacy And Redaction

The catalog record write path must use existing catalog model constructors and
store helpers. It must not persist or print:

- raw workflow YAML;
- raw draft YAML;
- source contents;
- package script bodies;
- dependency values or lockfile contents;
- CI logs;
- command output;
- provider payloads;
- parser payloads;
- absolute private paths;
- environment variables;
- credentials, authorization headers, private keys, token-like values;
- existing agent instruction bodies.

Debug output, JSON output, deserialization errors, duplicate-write errors, and
partial-integration errors must stay bounded and non-leaking.

## 14. Test Plan

Future implementation tests should cover:

- default promotion remains unchanged and writes no catalog record;
- `--catalog-root` without `--persist-catalog-record` is rejected;
- `--stewardship-decision-id` without `--persist-catalog-record` is rejected;
- dry-run with catalog persistence writes no files;
- promotion with `--persist-catalog-record` writes one active workflow file and
  one catalog record;
- persisted catalog record has active lifecycle status and active workflow path;
- catalog record cites verified stewardship decision id when supplied;
- catalog record discloses no stewardship citation when absent;
- supplied stewardship id must match workflow id;
- supplied stewardship id must match draft path;
- supplied stewardship id must match content hash;
- rejected/deferred/needs-changes decisions fail closed before active mutation;
- missing or corrupt stewardship record fails closed without leaking payload;
- duplicate catalog record write fails closed or follows an explicitly planned
  update mode;
- invalid catalog input fails before active workflow write;
- post-promotion catalog write failure produces explicit partial-integration
  status without runtime state;
- `catalog-status` reads the written catalog record;
- no raw workflow YAML, catalog JSON, command output, provider payloads, paths,
  or secrets are copied into output, errors, debug, or serialization;
- existing authoring promotion, steward-review, catalog-status, validation, and
  docs tests still pass.

## 15. Proposed Implementation Sequence

1. Add CLI parsing for `--persist-catalog-record`,
   `--stewardship-decision-id`, and catalog-root gating.
2. Add helper to load and verify a supplied stewardship decision for a promotion
   candidate.
3. Add helper to construct a bounded `WorkflowCatalogRecord` from promoted
   workflow context.
4. Wire the helper into `author workflow promote` only when
   `--persist-catalog-record` is present.
5. Add tests for default non-mutation and opt-in catalog write behavior.
6. Add tests for stale/mismatched stewardship decision records.
7. Add tests for partial-integration disclosure.
8. Update CLI and persistence docs.
9. Create implementation report.
10. Run maintainer review before archive metadata writes or stricter promotion
    enforcement.

## 16. Deferred Work

Deferred:

- requiring persisted stewardship for every promotion;
- workflow-declared catalog policy;
- enterprise steward/admin policy;
- catalog record update semantics;
- catalog repair;
- automatic rollback;
- archive metadata writes;
- draft deletion;
- runtime workflow registration;
- schemas;
- examples;
- provider calls;
- hosted collaboration;
- write-capable adapters;
- release posture changes.

## 17. Final Recommendation

The next implementation phase should add opt-in promotion catalog record writes
behind `--persist-catalog-record`.

The first implementation should allow but not require a verified persisted
stewardship decision id. That keeps single-user local promotion ergonomic while
making durable stewardship citation available for stricter future profiles.

The implementation must not add runtime registration, automatic promotion,
archive metadata writes, schemas, examples, provider calls, hosted behavior,
external writes, or release posture changes.
