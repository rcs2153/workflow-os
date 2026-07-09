# Workflow Catalog Repair Review CLI Write Blocker Fix Report

## 1. Executive Summary

The workflow catalog repair review CLI write blocker is fixed.

The maintainer review found that missing `--dry-run` and missing
`--persist-review` failed closed but used the generic `cli.usage` error code
instead of the repair-review-specific stable error codes required by the
accepted implementation plan.

The fix replaces those two generic usage failures with explicit stable CLI
codes and adds focused regression assertions.

## 2. Blocker Fixed

Fixed blocker:

- missing `--dry-run` now returns
  `cli.workflow_catalog.repair_review.requires_dry_run`;
- missing `--persist-review` now returns
  `cli.workflow_catalog.repair_review.requires_persist_review`;
- both failures remain non-mutating and bounded.

## 3. Scope Completed

- Updated the repair review CLI write command's missing explicit flag errors.
- Preserved the existing human-readable error messages.
- Added focused CLI test assertions for the stable error codes.
- Preserved existing successful repair review persistence behavior.
- Preserved duplicate, stale, invalid, unknown proposal, and secret-like input
  behavior.

## 4. Scope Explicitly Not Completed

- No repair apply mode.
- No automatic catalog repair.
- No catalog mutation expansion.
- No active workflow rewrites.
- No runtime workflow registration.
- No runtime state creation.
- No event or audit append.
- No report artifact generation.
- No workflow schema changes.
- No examples.
- No hosted/team catalog backend behavior.
- No provider calls.
- No local command/check execution.
- No write-capable adapter behavior.
- No release posture changes.

## 5. Implementation Approach

The implementation keeps the existing explicit flag checks at the CLI command
boundary and changes only the returned error values:

- `WorkflowOsErrorKind::Unsupported`;
- stable repair-review-specific error code;
- existing bounded message.

No parser broadening, helper redesign, store behavior change, or output shape
change was introduced.

## 6. Validation Boundary Summary

The command still requires:

- `--dry-run`;
- `--persist-review`;
- `--proposal-id`;
- `--review-id`;
- `--decision`;
- `--reviewer`;
- `--reason`.

Fresh proposal recomputation, exact proposal selection, model review
construction, stale proposal validation, duplicate rejection, and one-sidecar
persistence behavior are unchanged.

## 7. Privacy And Redaction Summary

The fixed errors do not echo proposal ids, review ids, reviewer values, reason
text, paths, source snippets, parser payloads, command output, provider
payloads, environment values, credentials, authorization headers, private keys,
tokens, or secret-like values.

The fix does not add any new output fields and does not print the reviewer
reason.

## 8. Test Coverage Summary

Focused regression coverage now asserts:

- missing `--dry-run` returns
  `cli.workflow_catalog.repair_review.requires_dry_run`;
- missing `--persist-review` returns
  `cli.workflow_catalog.repair_review.requires_persist_review`;
- missing explicit flag failures remain non-mutating.

Existing focused tests continue to cover persisted sidecar success, bounded JSON
and human output, unknown proposal rejection, duplicate rejection, and
secret-like reason non-leakage.

## 9. Commands Run And Results

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p workflow-cli --test cli author_workflow_catalog_repair_review
cargo test --workspace
npm run check:docs
```

Result: passed.

## 10. Governed Phase Metadata

- dogfood workflow id: `dg/blocker`
- run id: `run-1783555307998451000-2`
- approval id: `approval/run-1783555307998451000-2/fix-approved`
- approval outcome: granted by delegated maintainer
- out-of-kernel work: repository edits, Rust/doc validation, git, PR, and merge
  operations are performed by Codex/human execution layer; the kernel
  coordinated governance only.

## 11. Remaining Known Limitations

- Persisted repair reviews are still not repair apply permission.
- Optional approval, policy, evidence, validation, and work-report citations are
  not exposed on the CLI.
- Review supersession, replacement, deletion, and cleanup semantics remain
  unimplemented.
- Persisted reviews do not cite catalog-status snapshot ids yet.
- Hosted/team catalog persistence remains future work.

## 12. Recommended Next Phase

Recommended next phase: workflow catalog repair review CLI write blocker fix
review.

The review should verify that the two stable error-code blockers are fixed and
that no repair apply or broader catalog mutation behavior was introduced.
