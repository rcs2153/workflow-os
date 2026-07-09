# Workflow Catalog Repair Review CLI Write Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to the next roadmap phase.

The blocker fix replaces the two generic `cli.usage` failures identified in
the implementation review with repair-review-specific stable error codes. The
change is intentionally narrow, keeps the command fail-closed, preserves the
existing non-mutating behavior, and does not introduce repair apply behavior or
broader catalog mutation.

## 2. Scope Verification

The fix stayed within the approved blocker-fix scope.

Completed scope:

- missing `--dry-run` now returns
  `cli.workflow_catalog.repair_review.requires_dry_run`;
- missing `--persist-review` now returns
  `cli.workflow_catalog.repair_review.requires_persist_review`;
- focused CLI assertions cover both stable error codes;
- the blocker-fix report documents the fix and validation.

No accidental scope expansion found:

- no repair apply mode;
- no automatic catalog repair;
- no catalog mutation expansion;
- no active workflow rewrites;
- no draft or archive movement;
- no runtime workflow registration;
- no runtime state creation;
- no event or audit append;
- no report artifact generation;
- no workflow schema changes;
- no examples;
- no hosted/team catalog backend behavior;
- no provider calls;
- no local command/check execution;
- no write-capable adapter behavior;
- no release posture change.

## 3. Original Blocker Restatement

The CLI write implementation correctly required `--dry-run` and
`--persist-review`, but missing either flag returned the generic `cli.usage`
error path.

The accepted plan and implementation review required explicit stable codes:

- `cli.workflow_catalog.repair_review.requires_dry_run`;
- `cli.workflow_catalog.repair_review.requires_persist_review`.

The issue was an error-contract blocker rather than a safety blocker: the
command already failed closed and did not write review sidecars when either
explicit flag was absent.

## 4. Fix Assessment

The selected fix is minimal and idiomatic for the existing CLI code:

- the explicit flag checks remain at the command boundary;
- each missing flag now returns `WorkflowOsErrorKind::Unsupported`;
- each missing flag uses the required repair-review-specific stable code;
- existing bounded human-readable messages are preserved;
- no parser redesign, store-helper change, or output-shape change was added.

This keeps the CLI contract stable for users and tests while avoiding a broader
refactor.

## 5. Validation Boundary Assessment

The repair review command still requires explicit operator intent:

- `--dry-run`;
- `--persist-review`;
- `--proposal-id`;
- `--review-id`;
- `--decision`;
- `--reviewer`;
- `--reason`.

Fresh proposal recomputation, exact proposal selection, review construction,
stale proposal rejection, duplicate review rejection, and one-sidecar
persistence behavior remain unchanged.

The missing-flag failures remain non-mutating. The focused test still asserts
that no `.workflow-os/catalog` directory is created for the explicit-flag
failure path.

## 6. Error Handling Assessment

The blocker is fixed.

Verified stable error codes:

- missing `--dry-run` emits
  `cli.workflow_catalog.repair_review.requires_dry_run`;
- missing `--persist-review` emits
  `cli.workflow_catalog.repair_review.requires_persist_review`.

The errors do not include raw proposal ids, review ids, reviewer values, reason
text, paths, source snippets, parser payloads, command output, provider
payloads, environment values, credentials, authorization headers, private keys,
tokens, or secret-like values.

Existing CLI-specific error codes for unknown proposals, ambiguous proposals,
invalid decisions, invalid reviews, stale proposals, duplicate reviews, and
persistence failures are unchanged.

## 7. Privacy And Redaction Assessment

The fix adds no new output fields and does not print the reviewer reason.

The two fixed error paths are static bounded messages. They do not copy raw
workflow YAML, catalog payloads, source contents, command output, provider
payloads, parser payloads, CI logs, environment values, credentials,
authorization headers, private keys, tokens, or secret-like values.

Debug, stderr, and JSON behavior remain bounded by the existing CLI and model
constructors.

## 8. Test Quality Assessment

Focused regression coverage now protects the original blocker:

- missing `--dry-run` asserts the required stable code;
- missing `--persist-review` asserts the required stable code;
- missing explicit flags remain non-mutating.

Existing repair review CLI tests continue to cover:

- successful repair review sidecar persistence;
- bounded human output;
- bounded JSON output;
- unknown proposal rejection without writes;
- duplicate review id rejection without overwrite;
- secret-like reason rejection without leakage;
- no runtime state creation;
- no unrelated catalog sidecar directories.

No additional blocker-level test gaps remain for this fix.

## 9. Documentation Review

The blocker-fix report correctly states:

- the Work Catalog repair review CLI error-code blocker is fixed;
- persisted repair reviews are still not repair apply permission;
- repair apply mode is not implemented;
- automatic repair is not implemented;
- catalog mutation expansion is not implemented;
- workflow rewrites, runtime registration, schemas, examples, provider calls,
  writes, and release posture changes remain deferred.

The original implementation review remains intact and continues to document the
blocker that was found. The blocker-fix report records the fix-forward state
without erasing that review history.

## 10. Validation Commands

Review validation:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p workflow-cli --test cli author_workflow_catalog_repair_review
cargo test --workspace
npm run check:docs
```

Result: passed.

## 11. Governed Phase Metadata

- dogfood workflow id: `dg/review`
- run id: `run-1783556080777015000-2`
- approval id:
  `approval/run-1783556080777015000-2/review-scope-approved`
- approval outcome: granted by delegated maintainer
- out-of-kernel work: repository review, documentation, validation, git, and PR
  operations are performed by Codex/human execution layer; the kernel
  coordinated governance only.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Consider optional CLI citations for approval, policy, evidence, validation,
  and work-report references in a separately planned phase.
- Consider adding catalog-status snapshot references before any future repair
  apply planning.
- Consider an explicit unsafe catalog-root test for this specific subcommand if
  future catalog root handling changes.

## 14. Recommended Next Phase

Proceed to the next roadmap phase.

This blocker fix closes the known stable error-code issue for persisted repair
reviews. The repair review CLI write surface is now accepted as implemented,
with repair apply mode and automatic catalog mutation still explicitly
deferred.
