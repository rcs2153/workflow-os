# DocsCheck Local Handler Plan

Status: Explicit `DocsCheckLocalHandler` implemented and still non-default/non-CLI. Production-posture planning is documented in [DocsCheck Local Handler Production-Posture Plan](docs-check-production-posture-plan.md). No production/default registration, CLI exposure, workflow schema fields, automatic check execution, report artifacts, evidence attachment, side-effect boundary implementation, source writes, or release posture changes are authorized by this document.

## 1. Executive Summary

Workflow OS now has local check command contracts, canonical command-template binding, a test-only dogfood validation handler, a structured local check result model, an injectable process-runner boundary, and a reviewed fix for raw process-output debug leakage.

The next low-risk non-dogfood candidate is `DocsCheck`, backed by the canonical command template `npm run check:docs`.

This plan defined the implementation boundary for a `DocsCheck` local handler. The implemented `DocsCheckLocalHandler` remains explicitly constructed and non-default. It is not registered by default, exposed through CLI, added to workflow schema fields, persisted as a check result, attached to evidence, written as a report artifact, modeled as a side effect, or generalized into broader command execution. The production-posture plan records the decision to keep the handler explicit while deferring default registration and CLI/schema exposure.

## 2. Goals

- Add a precise implementation plan for the first non-dogfood local check handler.
- Keep local check execution explicit, allowlisted, and non-shell.
- Reuse the existing `LocalCheckResult`, `LocalCheckProcessRequest`, `LocalCheckProcessRunner`, and bounded output policy.
- Preserve existing local executor semantics and event ordering.
- Run only the canonical `DocsCheck` command template.
- Capture bounded, redaction-safe stdout/stderr summaries.
- Keep secrets, provider payloads, parser payloads, raw command output, and raw docs content out of persisted model surfaces.
- Prepare for future work-report or evidence citation without implementing either attachment.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- production local check registration;
- default handler registration;
- CLI handler exposure;
- workflow schema changes;
- arbitrary shell execution;
- user-supplied command text;
- caller-supplied additional arguments;
- automatic check execution;
- automatic Codex control through the kernel;
- report artifact writing;
- local check evidence attachment;
- command-output evidence policy;
- side-effect boundary implementation;
- source writes;
- write-capable adapters;
- live provider access;
- recursive agents;
- agent swarms;
- hosted/distributed runtime behavior;
- Level 3 or Level 4 autonomy;
- release posture changes.

## 4. Current Foundation

Implemented foundation:

- `LocalCheckCommandKind::DocsCheck` exists as model vocabulary.
- Canonical template binding maps `DocsCheck` to `npm run check:docs`.
- `LocalCheckCommandContract` rejects mismatched executable/argument templates.
- `LocalCheckExecutionPosture::AllowlistedHandlerOnly` remains rejected by serialized contract validation.
- `LocalCheckResult` stores validated, bounded summaries.
- `LocalCheckProcessRunner` supports deterministic tests without direct process execution.
- `LocalCheckProcessOutput` has redaction-safe `Debug`.
- The only executable handler today is `TestOnlyWorkflowOsValidateDogfoodHandler`.

Remaining limitations:

- no production `DocsCheck` handler registration;
- no production handler registry;
- no real sandbox;
- no declared npm cache/write policy in handler code;
- no CLI exposure;
- no workflow schema fields;
- no local check evidence attachment;
- no work-report integration for check results.

## 5. Candidate Handler

Candidate name:

- `TestOnlyDocsCheckHandler`, if the next implementation remains test-only;
- `DocsCheckLocalHandler`, only if the implementation explicitly remains non-default and reviewed as an allowlisted local handler.

Recommended first implementation posture:

- use a test-only or explicitly constructed handler, not default registration;
- continue rejecting serialized `AllowlistedHandlerOnly` contracts until a separate authorization posture review accepts otherwise;
- expose handler construction only to tests or narrow internal call sites.

Rationale:

- `npm run check:docs` is project-owned and narrow;
- it does not require provider credentials;
- it validates docs without changing source files;
- it still invokes Node/npm and therefore needs explicit environment, cache, timeout, and output policy before implementation.

## 6. Command Authority Rules

The future handler must:

- accept only `LocalCheckCommandKind::DocsCheck`;
- execute only the canonical executable/argument template: `npm run check:docs`;
- use executable plus fixed argument vector;
- never invoke a shell;
- never concatenate command strings;
- reject caller-supplied extra arguments;
- reject mismatched executable or argument values through the existing contract model;
- use an explicit working directory;
- use a sanitized, minimal environment;
- use a bounded timeout;
- use bounded output capture;
- return stable non-leaking errors.

The handler must not support:

- pipes;
- redirection;
- glob expansion;
- command substitution;
- command chaining;
- arbitrary package scripts;
- user-supplied command text;
- live provider credentials.

## 7. Working Directory And Tooling Policy

Recommended working directory:

- repository root.

Recommended executable policy:

- prefer an explicit executable path supplied by tests or construction, as the dogfood handler does;
- do not search arbitrary user paths inside the handler;
- if resolving `npm` by name is later required, document and test the path resolution policy separately.

Recommended Node/npm policy:

- use the repository-pinned toolchain path where available in tests;
- set `NPM_CONFIG_CACHE` to a repository-local or explicit cache directory if real process execution is used;
- do not inherit ambient npm tokens or registry credentials;
- do not run `npm install`, `npm ci`, or dependency mutation commands;
- do not authorize network access.

## 8. Environment Policy

The future handler should start from an empty environment and add only reviewed values.

Allowed candidate environment values:

- `PATH`, if required to locate Node/npm;
- `NPM_CONFIG_CACHE`, if explicitly set to a local non-secret cache path.

Forbidden:

- `NPM_TOKEN`;
- GitHub/Jira/CI provider tokens;
- authorization headers;
- private keys;
- registry credentials;
- unbounded inherited environment values;
- secret-like variable names or values.

All environment keys and values must pass existing local check environment validation.

## 9. Cache And Side-Effect Posture

`DocsCheck` is intended to be read-only against repository source, but Node/npm may read or write cache metadata.

Future implementation must decide one of:

- use an existing repository-local cache path and document it as permitted cache behavior; or
- use an injected fake runner for handler tests and defer real npm execution; or
- require the caller to supply a reviewed cache directory.

The handler must not:

- modify source files;
- write report artifacts;
- write workflow state outside normal explicit test execution events;
- install dependencies;
- update lockfiles;
- mutate package metadata;
- run arbitrary package lifecycle scripts beyond the fixed `check:docs` script.

If cache writes are permitted, they must be documented as cache-only behavior and must not be confused with source writes.

## 10. Output And Redaction Policy

The handler must use existing bounded output capture.

Rules:

- store only bounded stdout/stderr summaries in `LocalCheckResult`;
- mark truncation explicitly;
- reject secret-like output before result construction;
- use redaction-safe `Debug`;
- do not persist raw output;
- do not store command transcripts;
- do not copy raw docs content;
- do not copy raw parser payloads;
- do not attach command-output evidence in this phase.

Recommended timeout:

- start with a bounded docs-check timeout of 120 seconds unless current repository docs checks require a larger reviewed bound.

## 11. Failure Semantics

Recommended mapping:

- zero exit maps to `LocalCheckResultStatus::Passed`;
- non-zero exit maps to `LocalCheckResultStatus::Failed`;
- timeout maps to `LocalCheckResultStatus::TimedOut` with stable error code `local_check.handler.timed_out`;
- spawn/wait failure returns a stable non-leaking internal error;
- secret-like stdout/stderr fails closed with stable code `local_check.output.secret_like`.

The future handler must not:

- alter global executor semantics;
- append post-terminal events;
- convert handler failures into misleading project validation diagnostics;
- emit CLI output;
- write report artifacts.

## 12. Handler Registration Posture

Recommended first implementation:

- explicit construction only;
- no default registry entry;
- no CLI flag;
- no workflow schema field;
- no ambient replacement of `--mock-all-local-skills`.

Tests may explicitly register the handler with `LocalExecutor` to prove event ordering and output behavior.

Before production registration, a separate phase must decide whether `AllowlistedHandlerOnly` becomes valid for a narrow subset of command kinds or whether execution authorization remains outside serialized contracts.

## 13. Runtime And Event Boundary

The future handler should run only through existing `SkillHandler` and `LocalExecutor` mechanics when explicitly registered.

Allowed:

- normal skill invocation events from an explicitly run workflow;
- bounded `SkillOutput` values derived from `LocalCheckResult`;
- deterministic injected-runner tests;
- optional focused real-process smoke test only if it can be kept local, bounded, and non-mutating.

Not allowed:

- new runtime event kinds;
- automatic check execution;
- post-terminal event appending;
- local check result persistence;
- report artifact writes;
- evidence attachment;
- CLI rendering;
- workflow schema changes.

## 14. Evidence And Work Report Boundary

`DocsCheck` results should not attach evidence in the first handler implementation.

Future integration may cite stable local check result references from work reports after a separate plan reviews:

- local check result reference identity;
- command-output evidence policy;
- whether docs check output should be represented as validation/test evidence rather than raw command output;
- how missing or failed local checks appear in terminal reports.

The first handler should return or expose only bounded local check result summaries through existing skill output.

## 15. Security And Privacy Review Points

Before implementation, reviewers should verify:

- no shell invocation;
- no inherited secret environment;
- no npm auth token exposure;
- no network dependency;
- no raw output persistence;
- no raw docs content copying;
- no source writes;
- no lockfile/package mutation;
- stable non-leaking error codes;
- redaction-safe debug output for requests, process output, result, handler, and errors.

## 16. Test Plan

Future implementation tests should cover:

- `DocsCheck` handler rejects unsupported command kinds;
- handler uses canonical `npm run check:docs` argument vector;
- handler uses explicit repository root working directory;
- handler starts from sanitized environment;
- secret-like environment key is rejected;
- secret-like environment value is rejected;
- injected runner maps zero exit to `passed`;
- injected runner maps non-zero exit to `failed`;
- injected runner maps timeout to `timed_out`;
- injected runner spawn failure returns stable non-leaking error;
- secret-like stdout fails closed without leaking;
- secret-like stderr fails closed without leaking;
- oversized stdout/stderr is bounded or rejected according to existing result policy;
- `LocalCheckProcessOutput` debug remains redaction-safe;
- handler debug does not leak paths, command tokens, environment values, or output;
- explicit executor registration emits normal skill events only;
- no post-terminal events are appended;
- no report artifacts are written;
- no `StateBackend` writes beyond normal explicit workflow execution state in tests;
- no CLI output is introduced;
- existing local check, local executor, work report, evidence, diagnostic, adapter telemetry, and runtime tests still pass.

If a real-process test is added, it must be opt-in or carefully bounded so normal tests remain deterministic and do not depend on local npm installation state beyond repository-controlled tooling.

## 17. Documentation Updates For Implementation

The future implementation should update:

- this plan;
- [Broader Local Check Handler Plan](broader-local-check-handler-plan.md);
- [Self-Governed Validation/Check Plan](self-governed-validation-check-plan.md);
- [Test-Only Local Check Handler Plan](test-only-local-check-handler-plan.md), if wording around dogfood-only execution changes;
- [Roadmap](../../ROADMAP.md), if roadmap status changes;
- an end-of-phase report under `docs/concepts/`.

Docs must continue to state:

- no default production local check handler registration;
- no CLI exposure;
- no workflow schema fields;
- no automatic check execution;
- no report artifact writing;
- no evidence attachment;
- no side-effect boundary implementation;
- no source writes;
- no release posture change.

## 18. Open Questions

- Should the first `DocsCheck` handler be explicitly named test-only, or should it be an internal allowlisted handler with no default registration?
- Should real npm execution be included in a focused test or should all handler tests use injected runners?
- Should `NPM_CONFIG_CACHE` be mandatory for any real process execution?
- What cache directory is acceptable if npm writes cache metadata?
- Should `DocsCheck` use the bundled repository Node/npm path or accept an explicit npm executable path?
- Should `DocsCheck` output be represented only as `LocalCheckResult` or also produce a future validation/test citation?
- When should `AllowlistedHandlerOnly` stop failing closed for `DocsCheck` contracts?
- Should docs check failures later appear in work reports as validation and quality checks?

## 19. Final Recommendation

The implemented phase is: **explicit `DocsCheck` local handler, non-default and in-memory/test-scoped only**.

The implementation uses the existing local check result and process-runner infrastructure, supports injected-runner tests first, keeps real npm execution non-default and explicitly constructed, and preserves the no-CLI/no-schema/no-artifact/no-evidence/no-write posture.

Still not to be built:

- default handler registration;
- CLI handler exposure;
- workflow schema fields;
- automatic check execution;
- report artifact writing;
- evidence attachment;
- command-output evidence;
- side-effect boundary implementation;
- cargo/npm broader command families beyond `DocsCheck`;
- source writes;
- live provider access;
- release posture changes.
