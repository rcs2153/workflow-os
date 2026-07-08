# Governed Workflow Authoring Draft Cleanup And Supersession Plan

Status: Planned only. This plan follows the accepted active promotion
implementation review in [Governed Workflow Authoring Active Promotion
Implementation Review](../concepts/GOVERNED_WORKFLOW_AUTHORING_ACTIVE_PROMOTION_IMPLEMENTATION_REVIEW.md).

## 1. Executive Summary

Workflow OS can now promote one reviewed inactive draft from
`workflows/drafts/<name>.workflow.yml` into the active `workflows/` surface.
The first active-promotion slice intentionally preserves the inactive draft
after promotion.

The next question is what should happen to a promoted draft after the active
workflow file exists. Leaving drafts forever is safe but can confuse
maintainers and agents. Deleting drafts immediately is tidy but can erase useful
review context. This plan defines the next bounded phase for draft cleanup,
archive, and supersession semantics.

This plan does not implement cleanup behavior. It does not change active
promotion, create runtime state, persist steward approvals, add catalog state,
execute commands, call providers, add schemas, update examples, enable writes,
or change release posture.

## 2. Goals

- Define explicit post-promotion draft states.
- Avoid stale inactive drafts being mistaken for active governance.
- Preserve review/audit usefulness where possible.
- Keep cleanup local, deterministic, and reviewable.
- Avoid deleting user-authored material by default.
- Preserve the active workflow file as the loader-visible source of execution.
- Prepare for future persisted approvals and workflow catalog state without
  requiring them now.
- Keep output bounded and redaction-safe.
- Support local single-maintainer usage and future steward/admin workflows.

## 3. Non-Goals

Do not implement in this phase:

- cleanup commands;
- automatic cleanup after promotion;
- draft deletion;
- draft archive file movement;
- draft metadata mutation;
- persisted steward approval records;
- workflow catalog persistence;
- runtime state creation;
- workflow run creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- workflow schema changes;
- examples;
- hosted or distributed behavior;
- RBAC, IdP, notifications, or admin UI;
- write-capable adapters or provider mutation;
- release posture changes.

## 4. Current Boundary

Current implemented authoring path:

1. `workflow-os first-run` emits review-only recommendations.
2. `workflow-os first-run --recommendation <id>` explains a recommendation.
3. `workflow-os author workflow --from-recommendation <id> --dry-run` previews
   inactive authoring obligations.
4. `workflow-os author workflow --from-recommendation <id> --output
   workflows/drafts/<name>.workflow.yml` writes one inactive draft.
5. `workflow-os author workflow preflight --draft ...` checks promotability.
6. `workflow-os author workflow steward-review --draft ...` previews bounded
   steward review.
7. `workflow-os author workflow promote --draft ...` writes one active workflow
   file and preserves the draft.

Current non-capabilities:

- no draft cleanup command;
- no archive directory contract;
- no superseded draft marker;
- no persisted approval linkage to a promoted draft;
- no workflow catalog state;
- no automatic cleanup after promotion.

## 5. Draft State Vocabulary

Future implementation should use a bounded internal vocabulary before any schema
exposure:

- `active_candidate`: an inactive draft that may still be reviewed.
- `promoted_preserved`: a draft whose content was used to create an active
  workflow file and is retained for review context.
- `superseded_by_active`: a draft that should not be promoted again because an
  active workflow file already represents it.
- `archived`: a draft moved out of the active draft queue for historical review.
- `stale_candidate`: a draft whose content hash no longer matches a known
  promoted/reviewed state.

The first implementation should not add schema fields. It can derive these
states from file placement, active workflow id/path checks, and optional
sidecar-free conventions.

## 6. Recommended First Behavior

The smallest useful implementation should be a non-mutating inspection command:

```sh
workflow-os author workflow draft-status \
  --draft workflows/drafts/<name>.workflow.yml
```

It should report:

- draft path;
- candidate workflow id;
- current draft content hash;
- matching active workflow path if present;
- active workflow id conflict status;
- inferred draft state;
- recommended next action;
- non-mutation boundary flags.

This first command should not move, edit, delete, archive, or register anything.

## 7. Cleanup Policy Options

Future cleanup behavior should be planned after non-mutating status is reviewed.

Options:

- `preserve`: keep the draft in place and disclose that it has been promoted.
- `mark-superseded`: update or create a bounded marker that prevents accidental
  re-promotion while preserving context.
- `archive`: move the draft to a dedicated archive surface such as
  `workflows/drafts/archive/`.
- `delete`: remove the draft only with explicit maintainer intent.

Recommended posture:

- default to `preserve` plus explicit status disclosure;
- plan `archive` before `delete`;
- avoid in-place YAML mutation until a typed draft metadata contract exists;
- avoid automatic cleanup until persisted approval/catalog semantics exist.

## 8. Safety And Governance Requirements

Any future cleanup command must:

- require explicit CLI intent;
- validate the current project first;
- validate draft path safety;
- reject absolute paths and traversal;
- refuse to touch active workflow files;
- refuse to archive or delete unpromoted drafts by default;
- avoid copying raw draft YAML into output;
- use stable non-leaking errors;
- print whether files were written;
- preserve deterministic behavior.

If deletion is ever supported, it must require a separate explicit flag and
should likely remain outside the first cleanup implementation.

## 9. Relationship To Persisted Approvals

The current active promotion command uses same-process reviewer/reason input and
does not persist approval records.

Draft cleanup should not pretend that a preserved draft is a durable approval
record. Future persisted steward approval consumption should be able to cite:

- draft path;
- draft content hash;
- active workflow path;
- candidate workflow id;
- reviewer/approval reference;
- promotion time;
- cleanup/archive action if any.

Until that exists, draft cleanup should remain local authoring hygiene, not an
audit-grade approval archive.

## 10. Relationship To Workflow Catalog State

Loader-visible file placement is currently the source of active workflow
visibility.

Future workflow catalog state may need to know:

- source recommendation id;
- draft path;
- active path;
- active workflow id;
- content hash lineage;
- owner/steward approval references;
- supersession status.

This plan does not implement catalog state. The first cleanup/status helper
should be compatible with later catalog state by using stable identifiers and
bounded status codes.

## 11. Privacy And Redaction

Draft cleanup/status output must not copy:

- raw draft YAML;
- raw source contents;
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
- existing agent instruction bodies;
- steward review reason text.

Allowed output:

- relative draft path;
- relative active path;
- workflow id;
- content hash;
- status codes;
- warning/blocker codes;
- boundary flags.

## 12. Error Handling

Future implementation should fail closed for:

- missing project manifest;
- invalid project validation;
- unsafe draft path;
- missing draft file;
- draft parse failure;
- secret-like path or id material;
- active workflow path mismatch;
- ambiguous active match;
- archive/delete request outside approved cleanup policy.

Errors must use stable codes and avoid echoing raw paths or payloads beyond
bounded relative paths that have already passed path validation.

## 13. Test Plan

Future tests should cover:

- promoted draft status is reported as `promoted_preserved` or equivalent;
- unpromoted draft remains `active_candidate`;
- draft whose active workflow id already exists is not silently promotable;
- missing draft fails closed;
- unsafe paths fail closed without leakage;
- secret-like path/id material fails closed without leakage;
- output does not copy raw draft YAML;
- command writes no files;
- command creates no runtime state;
- command executes no commands;
- command calls no providers;
- JSON output is bounded;
- docs check passes.

If a later archive command is implemented, add tests for:

- archive writes exactly one destination file;
- archive refuses overwrite;
- active workflow file is untouched;
- unpromoted draft is not archived without explicit force;
- no runtime state or events are created.

## 14. Proposed Implementation Sequence

1. Implement non-mutating `author workflow draft-status --draft ...`.
2. Add focused status/non-leakage/non-mutation tests.
3. Review.
4. Plan archive command separately.
5. Plan persisted approval consumption separately.
6. Plan workflow catalog state separately.
7. Defer deletion until archive and catalog semantics are reviewed.

## 15. Open Questions

- Should the first command be named `draft-status`, `draft inspect`, or
  `promotion-status`?
- Should archive use `workflows/drafts/archive/` or a separate
  `.workflow-os/authoring/` surface?
- Should supersession be represented by file placement, marker file, or future
  typed metadata?
- Should cleanup require the same reviewer/reason pattern as promotion?
- Should cleanup ever delete drafts, or only archive them?
- How should future catalog state reconcile active workflow edits after
  promotion?
- Should preserved drafts be ignored by default in future recommendation scans?

## 16. Final Recommendation

Proceed next to a non-mutating draft status implementation.

The first implementation should inspect one draft and report whether it appears
active, promoted-preserved, superseded, stale, or still an active candidate. It
must not move, edit, delete, archive, register, run, persist, call providers,
add schemas, add examples, enable writes, or change release posture.
