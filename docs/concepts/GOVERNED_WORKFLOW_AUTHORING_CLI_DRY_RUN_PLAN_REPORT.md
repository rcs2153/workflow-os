# Governed Workflow Authoring CLI Dry-Run Plan Report

## 1. Executive Summary

This phase creates the governed workflow authoring CLI dry-run plan.

The plan defines a narrow CLI surface for turning an existing first-run recommendation into a reviewable inactive authoring preview without writing workflow files, registering workflows, executing commands, calling providers, creating runtime state, changing schemas, adding examples, enabling hosted behavior, or enabling writes.

## 2. Scope Completed

- Planned the `workflow-os author workflow --from-recommendation <id> --dry-run` boundary.
- Defined required dry-run inputs.
- Defined human output expectations.
- Defined optional preview JSON posture.
- Defined validation and fail-closed behavior.
- Defined privacy and redaction boundaries.
- Defined test coverage for the future implementation.
- Defined deferred work before file-writing or promotion.
- Updated roadmap and parent authoring plan links.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- CLI authoring command code;
- workflow file generation;
- workflow registration;
- workflow promotion or activation;
- workflow catalog storage;
- repository file writes;
- runtime state creation;
- local command execution;
- local check execution;
- provider calls;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. Planned CLI Boundary

Recommended first implementation:

```sh
workflow-os author workflow --from-recommendation <id> --dry-run
```

The command should be explicit, dry-run-only, non-mutating, and separate from `workflow-os run`.

## 5. Validation Boundary Summary

Future implementation should fail closed when:

- `--dry-run` is missing;
- the recommendation id is missing;
- the recommendation id is unknown;
- the recommendation id is invalid or secret-like;
- first-run recommendation derivation fails;
- proposal construction fails;
- output would require unsafe raw payload copying.

Errors should use stable codes and avoid echoing unsafe input values.

## 6. Privacy And Redaction Summary

The plan requires bounded safe metadata and static Workflow OS vocabulary only.

The future command must not copy source contents, manifest bodies, package script command bodies, dependency values, lockfile contents, CI logs, provider payloads, parser payloads, absolute private paths, environment values, credentials, token-like values, or existing agent instruction bodies.

## 7. Test Coverage Plan

The plan calls for focused tests covering:

- dry-run requirement;
- missing, unknown, and secret-like recommendation ids;
- inactive preview output;
- required authoring decisions;
- missing owner/escalation obligations;
- evidence/check, side-effect, and report/handoff obligations;
- explicit non-goals;
- non-mutation statements;
- no file writes;
- no runtime state creation;
- no provider calls;
- non-leakage of raw package scripts, source contents, and dependency values;
- existing first-run, CLI, scaffold, validation, runtime, and docs tests.

## 8. Governed Dogfood Summary

- Workflow: `dg/d`.
- Run ID: `run-1783397924825278000-2`.
- Approval ID: `approval/run-1783397924825278000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations; terminal status `Completed`.
- Scope: create planning doc and phase report for non-mutating governed workflow authoring CLI dry-run boundary.
- Out-of-kernel work disclosed: file editing, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 9. Validation Commands Run

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783397924825278000-2 --phase planning`: passed.

## 10. Remaining Known Limitations

- No authoring CLI command exists yet.
- No draft workflow file output exists.
- No catalog conflict checks exist.
- No workflow promotion path exists.
- No owner/escalation input handling exists for authoring.
- No workflow-declared authoring contract exists.

## 11. Recommended Next Phase

Recommended next phase: governed workflow authoring CLI dry-run implementation.

The implementation should remain local, explicit, dry-run-only, and non-mutating. It should reuse the accepted inactive draft proposal helper and require review before any future file-writing, catalog, or promotion path.
