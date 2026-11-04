# Hacker News Launch Admission Phase 3 Inspect Reference Blocker Fix Review

Status: accepted locally pending terminal-green exact-head CI

## Scope Reviewed

This focused maintainer and security review covered:

- projection of stable local-check references already present in persisted
  WorkReport citations;
- human and JSON inspect output boundaries;
- deterministic ordering and duplicate suppression;
- validation, redaction, and payload-exposure posture;
- compatibility with existing report-artifact output;
- focused and full validation evidence; and
- Phase 3 evaluation and recovery claims.

## Finding Corrected

### P1: stable references required projection-boundary control escaping

`WorkReportStableReference` rejects empty, over-bound, and secret-like values,
but it does not reject every control character. The initial human projection
printed a reference directly, which could have created additional terminal
lines. The existing hand-written JSON escaping helper covered backslashes,
quotes, and newlines but not all JSON control characters.

The reviewed implementation now:

- escapes every control character in human reference output through Rust's
  single-record default escaping;
- escapes quotes, backslashes, named JSON controls, and every remaining
  U+0000-U+001F value in JSON output; and
- directly tests both forms with newline, carriage return, tab, U+0001,
  backslash, and quote input.

This is projection hardening only. It does not change the stored report,
citation target, runtime state, report generation, or authority semantics.

## Accepted Posture

No P0 or unresolved P1 finding remains in the local diff.

References are selected only from typed `LocalCheckResult` citation targets,
sorted, and deduplicated. Human output adds one indented reference line beneath
its existing report-artifact summary. JSON output adds one deterministic array
to each artifact object. Existing fields and event output are unchanged.

The projection exposes no report section text, command output, repository
content, evidence body, provider payload, or redaction value. The underlying
stable reference remains bounded and secret-screened; projection escaping adds
the missing display-channel boundary.

The first clean-room pass, terminal replay, injected artifact-write failure,
same-run recovery, and uninstall-state preservation evidence are credible. A
new package must still be built from the corrected merged commit and exercised
from scratch before Phase 3 can be accepted.

## Validation Reviewed

Before the final projection-hardening correction:

- `cargo test --workspace --all-features`: passed;
- 176 CLI integration tests: passed;
- 773 Core unit tests: passed;
- 345 local-executor tests passed with one explicit opt-in test ignored;
- warnings-denied workspace rustdoc: passed;
- `npm run check`: passed;
- formatting, docs, and diff checks: passed; and
- warnings-denied Clippy found only an overlong human inspect function, which
  was extracted without behavior change and then passed.

After the correction, the direct escaping unit regression, exact first-run
artifact inspect/replay regression, complete workspace matrix, warnings-denied
Clippy, warnings-denied rustdoc, `npm run check`, formatting, docs, and diff
checks all passed. Exact-head CI remains the merge gate.

## Governed Review

- workflow: `dg/review`
- run: `run-1793750678208032000-2`
- approval: `approval/run-1793750678208032000-2/review-scope-approved`
- presentation proof: `presentation/5279ce130ddd397f`
- outcome: granted under delegated-maintainer authority
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record

The kernel governed review scope and approval. Diff inspection, security
analysis, shell commands, test execution, correction, and review writing were
performed outside the kernel and are disclosed here.
