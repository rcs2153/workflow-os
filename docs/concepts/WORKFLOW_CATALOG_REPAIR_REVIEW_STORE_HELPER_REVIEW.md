# Workflow Catalog Repair Review Store Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds the intended core store-helper boundary for local
repair proposal review sidecars. It is appropriately narrow, validates review
freshness against a supplied proposal before persistence, rejects duplicate
review ids, preserves deterministic read/list behavior, and avoids CLI write or
repair-apply behavior.

## 2. Scope Verification

The phase stayed within the approved store-helper scope.

Confirmed absent:

- no CLI repair review write command;
- no repair apply mode;
- no automatic catalog repair;
- no catalog record mutation through this helper;
- no active workflow rewrites;
- no draft/archive movement;
- no runtime workflow registration;
- no runtime state creation;
- no event or audit append;
- no report artifact generation;
- no workflow schema changes;
- no examples;
- no provider calls;
- no local check or command execution;
- no hosted/team catalog backend;
- no write-capable adapter behavior;
- no release posture change.

## 3. Store API Assessment

The new `LocalWorkflowCatalogStore` methods are appropriately minimal:

- `write_repair_review_record_if_absent`;
- `read_repair_review_record`;
- `list_repair_review_records`.

The API accepts a validated `WorkflowCatalogRepairProposalReview` plus a fresh
`WorkflowCatalogRepairProposal` for writes. That keeps proposal freshness at the
store boundary without turning persisted reviews into repair-apply permission.

The API remains local and explicit. It does not invent hidden runtime state,
load CLI inputs, mutate workflows, or call providers.

## 4. Storage Layout Assessment

Repair review records are stored under a separate `repair-reviews/` sidecar
directory using the existing encoded-id file-name policy.

This is the right layout for this phase because repair reviews are catalog
lifecycle metadata, not active workflow specs, stewardship records, archive
records, or runtime state. Tests verify that writing a repair review creates the
repair review sidecar directory without creating `workflows/`, `stewardship/`,
or `archives/`.

## 5. Fresh Proposal And Stale Review Assessment

`write_repair_review_record_if_absent` calls
`validate_workflow_catalog_repair_proposal_review_matches` before writing. A
stale proposal identity is mapped to the stable non-leaking store code
`workflow_catalog.repair_review_store.stale_proposal`.

This is the correct conservative boundary. Persisted review records remain
point-in-time governance sidecars and cannot silently authorize changed repair
proposals.

## 6. Duplicate And Replacement Assessment

Duplicate review ids are rejected before overwrite with
`workflow_catalog.repair_review_store.duplicate_review`.

Replacement, deletion, supersession, and cleanup semantics remain unimplemented,
which is correct for this phase.

Non-blocking nuance: the generic atomic publish helper can still return
`workflow_catalog_store.record_exists` if a same-id race happens after the
repair-review-specific preflight check. That does not permit overwrite or data
loss, but the CLI write phase should consider mapping that race path to the
repair-review-specific duplicate code for clearer operator UX.

## 7. Validation And Error Handling Assessment

The implementation relies on existing model constructors and serde validation
for record contents. Read and list paths fail closed on corrupt, invalid, or
storage-address-mismatched records. Error messages remain stable and do not echo
raw ids, paths, reviewer reasons, serialized payloads, command outputs, or
provider payloads.

The stale-proposal mapping intentionally hides the underlying proposal mismatch
details. That is appropriate because stale review diagnosis should be handled
through bounded status/review surfaces, not raw error text.

## 8. Privacy And Redaction Assessment

The helper does not read or copy raw workflow YAML, raw catalog payloads, source
contents, command output, provider payloads, parser payloads, CI logs,
environment values, credentials, authorization headers, private keys, tokens, or
secret-like values.

Tests cover invalid serialized repair review payload rejection without leaking a
secret-like reason value or review id through error text. Existing Debug posture
for the store remains redacted.

## 9. Test Quality Assessment

Focused test coverage is strong for this phase:

- write/read/list round trip;
- deterministic review-id ordering;
- separate `repair-reviews/` sidecar directory;
- duplicate rejection without overwrite;
- stale proposal rejection before persistence;
- invalid serialized repair review failure without leaking payload values;
- health summary count inclusion;
- existing catalog/stewardship/archive store behavior.

The tests prove behavior rather than construction. Remaining deeper coverage can
wait for the CLI write phase and apply-planning phases.

## 10. Documentation Review

Docs accurately say:

- repair review store-helper persistence is implemented;
- CLI repair review write behavior is not implemented;
- repair apply mode is not implemented;
- automatic repair is not implemented;
- catalog mutation, active workflow rewrites, runtime registration, schemas,
  examples, provider calls, hosted behavior, write-capable adapters, and release
  posture changes remain deferred.

The phase report includes governed dogfood metadata, validation commands, and
remaining limitations.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- In the future CLI write phase, map same-id atomic publish races to a
  repair-review-specific duplicate error where feasible.
- Before repair apply planning, define review supersession/replacement policy
  instead of overloading duplicate review ids.
- Consider later citation to catalog-status snapshot ids so persisted reviews
  can reference the status context that produced the proposal.

## 13. Recommended Next Phase

Recommended next phase: CLI repair review write planning.

The store-helper boundary is accepted. The next useful step is to plan the
explicit operator-facing write path that can select one fresh dry-run repair
proposal, construct a bounded review, persist exactly one sidecar, and still
avoid repair apply behavior.

## 14. Validation

Commands run for this review:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
```

Result: passed.

## 15. Governed Phase Metadata

- dogfood workflow id: `dg/review`
- run id: `run-1783551373756372000-2`
- approval id: `approval/run-1783551373756372000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- out-of-kernel work: code/document inspection, review document writing,
  validation commands, git, PR, and merge operations are performed by
  Codex/human execution layer; the kernel coordinated governance only.
