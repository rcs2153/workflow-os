# Generic Explicit Local-Check Profile Source Plan

Status: Implemented and reviewed; canonical-contract blocker fixed and
accepted.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Local Check Handler Default-Registration Plan](local-check-handler-default-registration-plan.md)
- [Canonical Local-Check Declaration Immutable Bundle Publication Plan](canonical-local-check-declaration-immutable-bundle-publication-plan.md)
- [Authoritative Local-Check Same-Call Composition Plan](authoritative-local-check-same-call-composition-plan.md)
- [Authoritative Quiet-Success CLI Preview Plan](authoritative-quiet-success-cli-preview-plan.md)
- [Authoritative Approval-Resume Report Completion Plan](authoritative-approval-resume-report-completion-plan.md)

## 1. Executive Summary

Workflow OS now has an accepted authoritative runtime path that can execute a
canonical local check, derive proportional governance, select the resulting
route, preserve approval freshness, and produce a terminal in-memory report.
The future quiet-success CLI preview cannot honestly expose that path yet
because its only production-shaped local-check handler is specific to the
Workflow OS repository's `npm run check:docs` command.

This plan defines one generic, explicit local-check profile source. The first
profile should validate the selected Workflow OS project with a fixed,
allowlisted `workflow-os validate` command executed from that project root.
The profile source must bind:

- one stable profile identifier;
- one canonical command contract;
- one stable skill identity and version;
- one explicit handler construction boundary;
- one immutable-bundle declaration inventory entry; and
- bounded, payload-free metadata for future CLI rendering.

The source must not infer commands from repository metadata, accept shell
strings, register handlers by default, or make project discovery execution
authority.

This plan does not implement the profile source, handler, CLI flag, automatic
check execution, schemas, persistence, artifacts, providers, OpenShell,
SideEffect execution, or writes.

## 2. Problem Statement

`LocalCheckRegistrationProfile` currently supports:

- `None`; and
- an explicitly supplied `DocsCheckLocalHandler`.

That boundary is safe but not generic. `DocsCheckLocalHandler` requires the
Workflow OS repository root, `package.json`, `scripts/check-docs.mjs`, an
explicit npm executable, and optional npm cache configuration.

The CLI currently creates either:

- an empty `LocalSkillRegistry`; or
- a mock fixture registry through `--mock-all-local-skills`.

It has no accepted source that can resolve an explicit profile into the same
contract, handler identity, and immutable declaration inputs consumed by the
authoritative executor.

A profile source that only registers a handler would be incomplete. The
immutable run bundle and authoritative reassessment must bind the exact command
contract that the handler executes. Registration, declaration publication, and
execution therefore need one common resolved profile.

## 3. Goals

The implementation phase should:

1. add one explicit, non-default profile-source model;
2. support one generic built-in profile:
   `workflow_os_project_validation`;
3. resolve that profile into one canonical command contract;
4. resolve one stable skill identity and version;
5. construct one explicit project-validation handler from caller-supplied
   runtime paths;
6. expose the exact declaration inventory required by immutable bundle
   publication;
7. preserve disabled network and source-read-only posture;
8. reject unsupported, ambiguous, or inconsistent selections before process
   execution;
9. keep debug output and errors path-safe and non-leaking; and
10. add one closed profile-to-authoritative-composition bridge so the resolved
    profile can enter the existing governance pipeline; and
11. leave ordinary registry and executor behavior unchanged.

## 4. Non-Goals

Do not add:

- CLI behavior or a quiet-success flag;
- automatic or default handler registration;
- arbitrary command strings or shell parsing;
- command discovery from `package.json`, `Cargo.toml`, CI files, source files,
  or agent instructions;
- automatic package-manager or toolchain selection;
- a generic arbitrary `SkillHandler` or command-runner authority surface;
- cargo, npm, TypeScript, contract, or integration profile families;
- workflow schema or SDK fields;
- scaffold or example changes;
- local-check result persistence;
- report artifacts or automatic WorkReport generation;
- provider, adapter, OpenShell, container, or sandbox execution;
- credential or environment discovery;
- network access;
- source writes or SideEffect execution;
- hosted/distributed behavior; or
- release posture changes.

## 5. Product Decision

The first generic profile should be:

```text
workflow_os_project_validation
```

Its logical command contract should be:

```text
workflow-os validate
```

executed with:

- the selected project root as the working directory;
- an explicitly supplied Workflow OS executable;
- a sanitized minimal environment;
- disabled network posture;
- source-read-only side-effect posture;
- bounded redacted output summaries; and
- no raw output persistence.

This profile is intentionally modest. It proves that a selected repository's
governance envelope is valid. It does not claim that project tests, builds,
lint, provider checks, or source-specific validations ran.

## 6. Why This Profile Is Generic

Every repository using Workflow OS has a project root and a governance
manifest. `workflow-os validate` is therefore meaningful across supported
repositories without requiring ecosystem-specific inference.

The profile does not depend on:

- the Workflow OS source repository;
- npm scripts;
- cargo workspace structure;
- a language runtime;
- provider credentials;
- network access; or
- repository source contents.

The profile is generic to Workflow OS projects, not a generic shell executor.

## 7. Candidate Core Model

The smallest justified model set is:

- `ExplicitLocalCheckProfileId`
- `ExplicitLocalCheckProfileSelection`
- `ResolvedExplicitLocalCheckProfile`
- `WorkflowOsProjectValidationLocalHandler`

The exact names may follow existing repository conventions during
implementation.

### `ExplicitLocalCheckProfileId`

A closed enum for the first implementation:

```text
WorkflowOsProjectValidation
```

It must not accept arbitrary string-backed profile IDs in the first slice.

### `ExplicitLocalCheckProfileSelection`

Explicit construction input:

- profile ID;
- Workflow OS executable path;
- project root;
- optional injected process runner for tests only.

It must not read ambient configuration or environment variables.

### `ResolvedExplicitLocalCheckProfile`

Validated output containing:

- profile ID;
- canonical command contract;
- canonical skill ID;
- canonical skill version;
- explicit non-default registration profile or isolated registry;
- immutable declaration inventory inputs; and
- bounded planned-handler metadata.

The resolved object must not expose raw command output, environment values,
absolute paths through `Debug`, or execution authority independent of its
validated handler.

### `WorkflowOsProjectValidationLocalHandler`

An explicit handler that:

- accepts only the canonical project-validation command kind;
- requires an existing Workflow OS executable;
- requires a project root containing `workflow-os.yml`;
- executes with that root as its working directory;
- uses a sanitized minimal environment;
- permits no network access;
- permits no source writes;
- captures bounded redacted summaries; and
- supports an injected process runner for deterministic tests.

## 8. Canonical Command Contract

Add one allowlisted command kind only if required by implementation:

```text
WorkflowOsValidateProject
```

Its fixed template should be:

```text
executable: workflow-os
arguments: [validate]
working_directory: selected project root
environment: sanitized minimal
network: disabled
side_effects: source read only
```

The selected project root is execution context, not a caller-supplied command
argument. It must not participate as unvalidated shell text.

The contract should retain the existing `ModelOnly` serialized execution
posture. Execution remains authorized only by explicit handler construction
and the authoritative executor path, not by changing serialized contract
posture.

## 9. Identity And Binding

The profile must define one stable identity:

```text
skill_id: local/workflow-os-validate
skill_version: v0
command_id: local-check/workflow-os-validate
command_kind: workflow_os_validate_project
```

Implementation may refine spelling to match current conventions, but all four
values must be deterministic and tested.

The same resolved profile must feed:

1. command-contract inventory used by immutable declaration publication;
2. handler registration;
3. selected-step authoritative preflight; and
4. bounded result/reference metadata.

No caller may pair the handler with a different command contract, skill
identity, or declaration fingerprint.

## 10. Resolution Boundary

Resolution should be explicit and side-effect free except for bounded
filesystem existence checks.

Allowed resolution checks:

- selected executable exists and is a file;
- project root exists and is a directory;
- `workflow-os.yml` exists at the project root;
- canonical contract validates;
- handler identity matches the profile;
- declaration inventory has exactly one matching contract.

Resolution must not:

- start a process;
- create directories;
- read raw manifest contents;
- discover commands;
- inspect source files;
- read credentials or environment values;
- mutate runtime state; or
- write artifacts.

## 11. Registration Posture

`LocalSkillRegistry::new()` must remain empty.

The resolved profile should either:

- produce an isolated registry containing exactly its handler; or
- register into a caller-provided empty registry through an API that rejects
  identity collisions.

The first option is preferred because it prevents silent replacement through
the existing general `register(...)` method.

The implementation must not add ambient default registration.

## 12. Immutable Bundle Integration

The profile source is useful only if it supplies the same canonical contract
used by immutable local-check declaration publication.

The first implementation should expose declaration inventory input but should
not change executor behavior.

Future authoritative CLI composition may then:

1. resolve the explicit profile;
2. load and validate the selected workflow;
3. publish the immutable run bundle using the profile's exact contract;
4. confirm the selected workflow step declares the matching requirement;
5. register the profile's exact handler;
6. execute the authoritative route; and
7. cite the bounded result in the report.

Missing or mismatched declarations must fail before process execution.

## 13. Authoritative Runtime Compatibility

The accepted authoritative runtime functions currently accept
`DocsCheckLocalHandler` directly. A new project-validation handler cannot enter
that path merely because a profile source resolves it.

The implementation phase must therefore add one narrow, closed bridge from
`ResolvedExplicitLocalCheckProfile` into the existing authoritative
composition pipeline.

The bridge must:

- accept only the closed built-in profile enum;
- use the profile's exact canonical command contract and handler;
- reuse existing execution-result, attestation, aggregate-fact, governance,
  approval, and report semantics;
- preserve one-check-per-route behavior;
- preserve existing `DocsCheck` public APIs and tests;
- fail before process execution on declaration or identity mismatch; and
- return the existing bounded local-check result shape.

The bridge must not:

- accept arbitrary `SkillHandler` implementations;
- accept command strings or caller-built process requests;
- dispatch by unvalidated string names;
- bypass immutable declaration binding;
- weaken current check freshness or approval reassessment; or
- make profile execution default.

Implementation may use a private closed enum or private trait to share the
already-reviewed process execution mechanics. It must not expose an open
public handler trait as execution authority in this phase.

Without this bridge, the profile source is not the final runtime prerequisite
for the quiet-success CLI preview.

## 14. CLI Relationship

This phase does not add CLI behavior.

After implementation and review, the quiet-success CLI preview may accept a
closed explicit selection such as:

```text
workflow-os run <workflow-id> \
  --authoritative-governance \
  --local-check-profile workflow-os-project-validation
```

That later CLI phase must:

- reject absent profile selection;
- reject unknown profile values;
- reject workflows without the exact declared check requirement;
- preserve ordinary `run` behavior;
- avoid self-approval;
- render quiet, visible, approval, and denial outcomes honestly; and
- never imply that project-specific tests ran.

## 15. Relationship To Safe Repo Metadata

Safe metadata discovery may recommend future profiles, but it must not select
or authorize one.

For example, detecting `package.json` may support a review-only recommendation
for a future npm profile. It must not cause `npm test` or any package script to
execute.

Inference may propose governance. Explicit validated profile selection grants
the first execution opportunity. Core policy and authority still decide
whether execution may proceed.

## 16. Relationship To Proportional Governance

The profile source supplies authoritative check facts. It does not choose the
governance route.

Core remains responsible for:

- deriving the assessment from immutable workflow facts and fresh runtime
  facts;
- preserving explicit profile/policy/steward minima;
- selecting quiet proceed, visible disclosure, approval, or denial;
- escalating monotonically when facts change; and
- refusing incomplete or ambiguous evidence.

Visible disclosure remains an output/presentation obligation independent of
whether execution blocks. The current two-axis model already represents this
separation; this phase should not collapse disclosure into execution posture.

## 17. Privacy And Redaction

The profile source and handler must not expose:

- absolute executable or project paths in `Debug`;
- manifest or source contents;
- raw stdout or stderr;
- environment values;
- tokens, credentials, authorization headers, or private keys;
- command-line payloads beyond fixed canonical vocabulary; or
- secret-like caller values in validation errors.

Errors must use stable codes and describe the failed boundary without echoing
the rejected value.

## 18. Failure Semantics

Fail before process execution when:

- the profile is unsupported;
- the executable or project root is invalid;
- the manifest is absent;
- the canonical contract is invalid;
- handler and contract identities differ;
- the immutable declaration inventory is missing, duplicated, or mismatched;
  or
- registration would collide.

After a process starts, use existing local-check result and authoritative
composition semantics. Do not create a partial resolved profile, fake result,
or reusable authority object after failure.

## 19. Test Plan

Future focused tests should prove:

1. explicit project-validation selection resolves one profile;
2. default selection or registry remains empty;
3. unsupported profile wire values fail closed;
4. canonical command identity and arguments are fixed;
5. project root is working context, not shell text;
6. missing executable fails before process execution;
7. missing project root or manifest fails before process execution;
8. `Debug` redacts executable and project paths;
9. errors do not leak secret-like paths or values;
10. resolved skill and command identities are deterministic;
11. declaration inventory contains exactly the resolved contract;
12. handler registration contains exactly the resolved skill;
13. handler/contract mismatch fails closed;
14. registration collision cannot silently replace a handler;
15. injected-runner success maps to a bounded passed result;
16. injected-runner failure and timeout remain structured;
17. raw stdout/stderr are not serialized or persisted;
18. no network or source-write posture is introduced;
19. no runtime state, event, report, or artifact is created by resolution;
20. existing `DocsCheck` explicit profile behavior remains unchanged;
21. the resolved profile enters authoritative composition through the closed
    bridge;
22. the bridge rejects contract, skill, and declaration mismatches before
    process execution;
23. the bridge preserves quiet, visible, approval, denial, and report route
    semantics;
24. no second report-only check is executed;
25. existing immutable-bundle and authoritative local-check tests pass; and
26. `cargo test --workspace` passes.

## 20. Documentation Plan

Implementation should update:

- this plan;
- `ROADMAP.md`;
- relevant local-check concept documentation; and
- an implementation report and focused review.

Docs must state:

- the profile source is explicit and non-default;
- only project governance validation is supported initially;
- project-specific tests are not inferred or executed;
- CLI exposure is still not implemented until separately scoped;
- raw output is not persisted;
- providers, OpenShell, SideEffect execution, and writes are not implemented by
  this phase.

## 21. Implementation Sequence

1. Add the closed profile ID and canonical project-validation command kind.
2. Add the explicit project-validation handler with injected-runner tests.
3. Add the resolved profile source that binds contract, skill identity,
   handler registration, and declaration inventory.
4. Add the closed profile-to-authoritative-composition bridge without changing
   existing public `DocsCheck` APIs.
5. Add collision, privacy, route, and no-process-on-preflight-failure tests.
6. Run focused and workspace validation.
7. Complete a maintainer review.
8. Only then implement the explicit authoritative quiet-success CLI preview.

## 22. Open Questions

- Should the resolved profile produce an isolated registry or use a
  collision-rejecting registration API?
- Should the project-validation handler invoke `workflow-os validate` from the
  project root or pass `--project-dir` explicitly?
- Should the current executable be supplied by the future CLI or resolved by a
  CLI-only boundary?
- Does the initial profile require a workflow-authored local-check declaration,
  or should a future CLI-owned workflow use a built-in immutable declaration?
- Should profile IDs become versioned before a second profile family exists?
- How should future ecosystem-specific profiles be proposed from safe metadata
  without becoming inferred execution authority?

## 23. Final Recommendation

Proceed with the **generic explicit local-check profile source implementation**
as a Core-only, non-default phase that includes the narrow closed
profile-to-authoritative-composition bridge.

The first profile should validate the selected Workflow OS project and bind one
fixed command contract, one handler identity, and one immutable declaration
inventory. Do not implement the quiet-success CLI flag until this source is
implemented and accepted.

## 24. Implementation Status

The Core-only phase is implemented.

The implementation adds:

- the closed `workflow_os_project_validation` profile ID;
- the fixed `workflow-os validate` command contract;
- the stable `local/workflow-os-validate` skill identity at version `v0`;
- an explicit `WorkflowOsProjectValidationLocalHandler`;
- a resolved profile that binds the handler, command contract, skill identity,
  and immutable declaration inventory;
- collision-rejecting explicit registry installation;
- a private closed authoritative-handler boundary shared by the existing
  `DocsCheck` path and the new project-validation profile;
- an additive authoritative route helper for resolved profiles; and
- an additive report-bearing helper that cites the same-call project-validation
  result without executing a second check.

Implementation resolved the open questions conservatively:

- explicit profiles use collision-rejecting registration rather than a
  replacement-capable registry path;
- the project-validation command runs from the selected project root;
- the executable is supplied explicitly by the caller;
- the selected workflow must declare the exact canonical local-check contract;
  and
- profile IDs remain a closed unversioned enum until a second profile family
  creates a concrete versioning requirement.

CLI exposure, default registration, inferred repository commands, project test
execution, persistence, artifacts, providers, OpenShell, SideEffect execution,
writes, hosted behavior, and release changes remain unimplemented.

See the
[Generic Explicit Local-Check Profile Source Report](../concepts/GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_SOURCE_REPORT.md)
for implementation and validation details.

Focused implementation review found one blocker: the public project-validation
handler constructor validates command kind and broad posture but does not prove
the supplied contract equals the complete canonical contract. The profile must
reject any caller-modified argument or other contract field before CLI
exposure. See the
[Generic Explicit Local-Check Profile Source Review](../concepts/GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_SOURCE_REVIEW.md).

The canonical-contract blocker is now fixed. Public handler construction
requires complete equality with the built-in contract and rejects drift before
process execution. See the
[Generic Explicit Local-Check Profile Canonical Contract Blocker Fix Report](../concepts/GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_CANONICAL_CONTRACT_BLOCKER_FIX_REPORT.md).

Focused blocker-fix review accepts the complete canonical equality boundary
without remaining blockers. See the
[Generic Explicit Local-Check Profile Canonical Contract Blocker Fix Review](../concepts/GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_CANONICAL_CONTRACT_BLOCKER_FIX_REVIEW.md).
