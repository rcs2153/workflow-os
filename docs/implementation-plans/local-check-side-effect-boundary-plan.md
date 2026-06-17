# Local Check Side-Effect Boundary Plan

Status: Model-only boundary implemented. The local check side-effect boundary vocabulary and validation are implemented in `workflow-core` as a model-only layer. This plan does not implement live local check execution, default registration, CLI behavior, workflow schema fields, command-output evidence, local check evidence attachment, report artifact auto-writing, persistence changes, writes, or release posture changes.

## 1. Executive Summary

Workflow OS can now dogfood a real `DocsCheckLocalHandler` through explicit profile registration and injected-runner tests. That proves the local check handler boundary without making local command execution ambient.

The next question is how Workflow OS should classify and constrain local check side effects before live npm runs, cargo checks, broader handlers, or default registration are considered.

This plan defines a local-check-specific side-effect boundary. It is narrower than the future generic side-effect boundary ADR for write-capable adapters. It focuses on local validation/check commands, cache/build/temp writes, network posture, environment posture, and source-tree protection.

This plan does not implement runtime behavior. It does not authorize live npm smoke tests, cargo handlers, true default registration, CLI exposure, workflow schema fields, command-output evidence, local check evidence attachment, persistence, report artifacts, writes, or release posture changes.

## 2. Goals

- Define a safe side-effect vocabulary for local validation/check commands.
- Preserve explicit local command authority.
- Distinguish repository source writes from cache, build, and temp writes.
- Avoid pretending local checks are side-effect-free when toolchains may touch caches.
- Keep source-tree mutation forbidden unless a future write phase explicitly authorizes it.
- Keep network and credential behavior denied by default.
- Define directory policy for repository roots, caches, build outputs, and temp directories.
- Preserve current local executor, report, audit, and event semantics.
- Prepare a small future model-only implementation prompt.
- Keep future live execution reviewable and testable.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- live local command execution;
- live npm smoke tests;
- cargo, TypeScript, contract, integration, or provider check handlers;
- true default handler registration;
- CLI handler exposure;
- workflow schema fields;
- automatic check execution;
- command-output evidence;
- local check evidence attachment;
- automatic report artifact writing;
- local check result persistence;
- generic side-effect records;
- write-capable adapters;
- source writes;
- provider calls;
- recursive agents;
- agent swarms;
- hosted or distributed runtime behavior;
- release posture changes.

## 4. Current Baseline

Implemented:

- `LocalCheckCommandKind` allowlisted command vocabulary;
- canonical command-template binding;
- `LocalCheckSideEffectClass` with `NoSourceWrites`, `BuildOrCacheWrites`, and `Unclassified`;
- `LocalCheckNetworkPolicy::Disabled`;
- `LocalCheckEnvironmentPolicy` vocabulary;
- bounded `LocalCheckResult`;
- injectable `LocalCheckProcessRunner`;
- explicit production-shaped `DocsCheckLocalHandler`;
- explicit non-default docs-check registry profile;
- dogfood workflow checkpoint for `local/check-docs`;
- injected-runner dogfood tests for real `DocsCheckLocalHandler` execution.

Implemented in the model-only boundary phase:

- `LocalCheckSideEffectKind`;
- `LocalCheckSideEffectBoundary`;
- validation for source-read-only, cache/build/temp directory requirements, source-write rejection, network-access rejection, unclassified fail-closed behavior, secret-like directory rejection, and redaction-safe debug output;
- contract-level derivation of a fine-grained side-effect boundary from the existing coarse `LocalCheckSideEffectClass` and permitted output directories.

Still not implemented:

- live npm smoke test posture;
- source/cache/build/temp directory enforcement model beyond current handler validation;
- default registration;
- CLI exposure;
- workflow schema activation;
- command-output evidence;
- local check evidence attachment;
- broader cargo/npm check handlers.

## 5. Why This Boundary Is Needed

Validation/check commands are not automatically read-only.

Examples:

- `npm run check:docs` is intended to be source-read-only, but npm can touch cache directories.
- `cargo fmt --all --check` should not rewrite source files, but toolchain execution may touch toolchain state.
- `cargo clippy` and `cargo test` can write build artifacts under `target/`.
- tests can perform filesystem, network, or environment behavior unless constrained.

Without a side-effect boundary, Workflow OS could accidentally classify a command as safe because it does not mutate source files while ignoring cache writes, build artifacts, network attempts, or credential exposure.

## 6. Local Check Side-Effect Taxonomy

Recommended local-check side-effect taxonomy:

| Class | Meaning | Current posture |
| --- | --- | --- |
| `source_read_only` | May read repository source and config; must not write repository source, generated source, lockfiles, or project metadata. | Desired posture for `DocsCheck`. |
| `cache_write_only` | May write only to explicitly supplied cache directories outside protected source paths. | Needed before live npm docs check. |
| `build_output_write` | May write declared build artifacts such as `target/`, never source files. | Needed before clippy/test handlers. |
| `temp_write_only` | May write to an explicit temp directory with cleanup/disclosure policy. | Future, if handlers need temp space. |
| `source_write` | May modify repository source files. | Reject for local check phases. |
| `network_access` | May use network. | Reject for local check v1 unless separately approved. |
| `unclassified` | Side effects are unknown or insufficiently constrained. | Fail closed. |

The existing `LocalCheckSideEffectClass` remains a coarse compatibility model. The model-only implementation added `LocalCheckSideEffectBoundary` as a separate validated wrapper so existing serialized contract shape remains stable while finer-grained validation is available.

## 7. Command Family Classification

| Command family | Current classification recommendation | Future implementation status |
| --- | --- | --- |
| `WorkflowOsValidateDogfood` | `source_read_only`; no cache/build writes expected. | Existing explicit test-only handler remains acceptable. |
| `DocsCheck` | `source_read_only` plus explicit `cache_write_only` allowance for npm cache if live execution is used. | Plan model boundary first; live smoke remains deferred. |
| `CargoFmtCheck` | `source_read_only` intent, but toolchain/cache behavior must be declared. | Defer. |
| `CargoClippyWorkspace` | `build_output_write` under `target/`; no source writes. | Defer. |
| `CargoTestWorkspace` | `build_output_write` under `target/`; test side effects require additional constraints. | Defer. |
| `TypeScriptCheck` | likely cache/build output writes depending on scripts. | Defer. |
| `ContractCheck` | likely generated/cache/build outputs depending on scripts. | Defer. |
| `IntegrationCheck` | likely broader filesystem/provider/network posture. | Defer. |
| Live provider smoke tests | network and credentials. | Reject for local check v1. |
| Arbitrary user commands | unknown/unbounded. | Reject. |

## 8. Directory Policy

Future local check boundary modeling should distinguish:

- repository root: readable, but protected from source writes;
- protected source paths: source files, docs, examples, specs, lockfiles, manifests, and project metadata;
- build output directories: declared directories such as `target/`;
- package/tool caches: explicit cache directories such as `NPM_CONFIG_CACHE`;
- temp directories: explicit temp roots, not ambient system temp by default;
- report artifact directories: out of scope for local checks unless a separate artifact phase authorizes them;
- state backend directories: not writable by local check handlers except through normal executor event behavior.

Rules:

- Protected source paths must not be written by local check handlers.
- Cache/build/temp directories must be explicit, bounded, and redaction-safe.
- Cache/build/temp paths must not contain secret-like segments.
- Handlers must not infer write directories from ambient environment.
- Handlers must not create or clean arbitrary user files.
- Cleanup semantics must be explicit before any handler owns temp output.

## 9. Environment And Network Policy

Default posture:

- start from an empty environment;
- add only explicitly allowed non-secret variables;
- reject secret-like environment variable names and values;
- do not pass provider tokens, registry credentials, authorization headers, private keys, or broad user environment;
- keep `LocalCheckNetworkPolicy::Disabled` for v1 local checks;
- treat any network requirement as a separate planning and review boundary.

`DocsCheck` live execution, if later approved, should require an explicit npm executable path and explicit cache directory. It should not search user `PATH`, install dependencies, run `npm ci`, or read registry credentials.

## 10. Runtime, Event, Audit, And Report Boundary

Local check side-effect boundary modeling must not change runtime semantics.

Allowed future behavior:

- existing workflow events for explicitly invoked workflow steps;
- bounded `LocalCheckResult` values;
- stable local check result references;
- WorkReport citations to supplied local check result references.

Not allowed in this planning phase:

- new event kinds;
- post-terminal event appends;
- automatic check execution;
- automatic report generation;
- automatic artifact writing;
- audit events outside existing executor paths;
- CLI output;
- persistence changes.

## 11. Evidence Boundary

Local check side-effect planning does not authorize evidence attachment.

Current posture:

- WorkReports may cite supplied `LocalCheckResultReference` values.
- `EvidenceKind::CommandOutput` remains policy-planned but unimplemented.
- Local check handlers must not create `EvidenceReference` values implicitly.
- Missing check result references should remain explicit section text, not fabricated evidence.

Any future local check evidence attachment must follow command-output evidence policy and a separate attachment validator.

## 12. Failure Semantics

Recommended future behavior:

- `unclassified` side effects fail closed before execution.
- Missing required explicit cache/build/temp directory fails closed.
- Directory policy violations fail closed.
- Network-required checks fail closed while network policy is disabled.
- Secret-like environment or path values fail closed.
- Source write detection, if implemented, fails the check and returns a stable non-leaking error.
- Cache/build/temp writes may be disclosed in reports only as bounded posture text or stable references.
- Report generation failures must remain separate from workflow execution failures.

Error messages must use stable codes and must not include raw paths, environment values, command output, source snippets, parser payloads, provider payloads, tokens, or credentials.

## 13. Test Plan For Future Implementation

A future model-only or boundary implementation should test:

- all local check side-effect classes are representable;
- `unclassified` checks fail closed;
- source-write authorization is rejected for local check phases;
- cache-write-only checks require explicit permitted cache directories;
- build-output-write checks require explicit permitted output directories;
- secret-like directory names are rejected without leaking;
- environment allowlist rejects secret-like names and values;
- network-disabled posture rejects network-required checks;
- `DocsCheck` can declare source-read-only plus explicit npm cache allowance without enabling default registration;
- cargo clippy/test remain deferred until build-output policy is accepted;
- errors use stable codes and do not leak paths or secrets;
- Debug and serialization are redaction-safe if new serializable model types are added;
- existing local check, local executor, WorkReport, EvidenceReference, Diagnostic, adapter telemetry, and runtime tests still pass.

If source-write detection is introduced later, tests must use a controlled temp repository fixture and must not delete or rewrite user files.

## 14. Proposed Implementation Sequence

Recommended next phases:

1. Review the model-only local check side-effect boundary.
2. Revisit `DocsCheck` live smoke posture with explicit npm cache policy.
3. Only after review, consider a narrow opt-in live docs-check smoke.
4. Defer cargo/clippy/test handlers until build-output policy is accepted.
5. Defer default registration, CLI exposure, workflow schema fields, command-output evidence, and local check evidence attachment.

## 15. Open Questions

- Should `LocalCheckSideEffectClass` be extended, or should a new `LocalCheckSideEffectBoundary` model compose with it?
- Should cache writes be allowed only outside the repository, or may a repo-local ignored cache directory be allowed?
- Should build output directories such as `target/` be treated as safe only when already ignored by Git?
- Should future source-write detection compare file hashes before and after execution, or rely on directory policy only?
- Should a live `DocsCheck` smoke be opt-in through tests, a local script, or deferred until CLI exposure planning?
- Should local check boundary modeling be separate from the generic side-effect boundary ADR, or should it feed that ADR as a concrete local-command slice?
- How should this boundary interact with future Composable Harness Contracts and typed handoffs?

## 16. Final Recommendation

The next phase should be: **local check side-effect boundary model review**.

The review should verify the model-only boundary remains backward-compatible, redaction-safe, and non-executing. It should also confirm that live commands, default registration, CLI behavior, schema fields, evidence attachment, local check result persistence, report artifacts, generic side-effect records, and writes remain unimplemented.
