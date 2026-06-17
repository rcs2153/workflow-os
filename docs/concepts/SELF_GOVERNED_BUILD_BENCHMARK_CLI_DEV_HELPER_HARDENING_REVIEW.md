# Self-Governed Build Benchmark CLI/Dev-Helper Hardening Review

## 1. Executive Verdict

Hardening accepted; return to kernel primitive roadmap work.

The hardening phase closes the non-blocking follow-ups from the CLI/dev-helper review. It adds targeted tests, removes unsupported-command echoing, and fixes script entrypoint detection across symlinked temp paths without expanding the helper into public CLI behavior or runtime automation.

No blocker was found.

## 2. Scope Verification

The phase stayed within approved helper hardening scope.

No accidental implementation was found for:

- stable public Workflow OS CLI benchmark commands;
- automatic runtime report generation;
- runtime result exposure changes;
- CLI report rendering;
- report artifact writing;
- automatic report artifact writing;
- automatic local check execution;
- default `DocsCheckLocalHandler` registration;
- arbitrary shell execution;
- command-output evidence attachment;
- workflow schema changes;
- workflow-declared benchmark behavior;
- workflow-declared hooks;
- runtime hook configuration;
- hook warning/skipped continuation;
- approval evidence attachment;
- reasoning lineage or claim graph;
- side-effect boundary enforcement;
- write-capable adapters;
- repository writes from inside the kernel;
- recursive agents;
- agent swarms;
- hosted or distributed runtime claims;
- production self-hosting claims;
- Level 3 or Level 4 autonomy claims;
- release posture changes.

## 3. Hardening Assessment

The hardening is appropriately narrow.

Implemented hardening:

- unsupported helper commands now return `dogfood.helper.unsupported` without echoing caller-supplied command text;
- entrypoint detection uses real paths so direct script execution works when temp paths traverse symlinks such as macOS `/tmp` and `/private/tmp`;
- `status` dry-run command shape is tested;
- `inspect` dry-run command shape is tested;
- `prompt` output boundary is tested;
- missing binary plus `--no-build` fails closed and is tested;
- unsupported-command non-leakage is tested.

The helper still exposes the same repo-local surface through `npm run dogfood:benchmark`.

## 4. Runtime And CLI Boundary Assessment

Runtime and CLI boundaries remain intact.

The helper still wraps existing generic CLI commands and does not introduce a stable Rust CLI subcommand. It does not bypass kernel validation, synthesize runtime state, create reports, write artifacts, register local check handlers, append events directly, or change workflow pass/fail semantics.

Approval remains explicit. The helper still requires run ID, approval ID, and reason for `approve`, and it still avoids a one-shot validate/start/approve flow.

## 5. Privacy And Error Handling Assessment

Privacy posture improved.

Unsupported command errors no longer echo arbitrary command text. That closes the only non-blocking leak-hardening issue from the previous review.

Existing safeguards remain in place:

- approval reasons are redacted in displayed commands;
- repo-root paths are displayed repo-relative where possible;
- secret-like approval metadata is rejected before command construction;
- missing binary errors use a stable helper error label;
- the helper does not create evidence, WorkReports, report artifacts, or command-output evidence.

## 6. Test Quality Assessment

Test coverage is now strong for the repo-local helper.

Covered:

- `start` dry-run command shape;
- `status` dry-run command shape;
- `inspect` dry-run command shape;
- explicit approval reason requirement;
- approval reason redaction;
- secret-like approval metadata rejection;
- helper boundary command output;
- prompt boundary output;
- repo-relative displayed command paths;
- secret-like value detection;
- unsupported command non-leakage;
- missing binary with `--no-build` failing closed.

Remaining non-blocking gap:

- no end-to-end `validate/start/approve/inspect` helper smoke test. This remains acceptable because the helper is a thin wrapper and the underlying CLI/runtime paths are already covered. A future smoke test can be added if helper usage becomes part of a CI or release readiness gate.

## 7. Documentation Review

Documentation is consistent with the hardening phase.

Docs state that:

- the helper remains repo-local development tooling;
- it is not stable public product CLI behavior;
- automatic runtime report generation is not implemented;
- report rendering and report artifact writing are not implemented;
- automatic local check execution is not implemented;
- default `DocsCheckLocalHandler` registration is not implemented;
- arbitrary command execution is not implemented;
- schemas, writes, reasoning lineage, recursive agents, hosted execution, production self-hosting, and Level 3/4 autonomy are not implemented.

## 8. Blockers

No blockers.

## 9. Non-Blocking Follow-Ups

- Consider a bounded end-to-end helper smoke test only if the helper becomes part of CI or release readiness.
- Keep helper behavior repo-local unless a future plan explicitly promotes a generic product CLI surface.

## 10. Recommended Next Phase

Recommended next phase: **commit helper hardening, then return to kernel primitive roadmap work**.

After commit, the strongest next roadmap candidate is **hook disclosure model implementation** because warning/skipped hook continuation, stronger dogfood checkpoints, and future harness governance depend on bounded disclosure semantics.

## 11. Validation

- `npm run test:dogfood-helper`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.
