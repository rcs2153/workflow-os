# Governed Workflow Authoring File Output Plan Report

## 1. Executive Summary

This phase planned the next governed workflow authoring mutation boundary: an explicit future file-output path for inactive draft workflow files.

The plan builds on the accepted `workflow-os author workflow --from-recommendation <id> --dry-run` implementation. It defines how a future implementation should move from preview-only authoring obligations toward a reviewable draft file while preserving safety: explicit output path, inactive lifecycle, conflict checks, no overwrite, no registration, no promotion, no command execution, no provider calls, no runtime state, and no active governance.

No implementation occurred in this phase.

## 2. Scope Completed

- Created [Governed Workflow Authoring File Output Plan](../implementation-plans/governed-workflow-authoring-file-output-plan.md).
- Defined the recommended future CLI shape for explicit draft output.
- Defined output location rules.
- Defined inactive draft lifecycle posture.
- Defined conflict handling requirements.
- Defined allowed and forbidden inputs.
- Defined draft content policy.
- Defined promotion boundary.
- Defined stable error-code candidates.
- Defined privacy and redaction requirements.
- Defined documentation requirements.
- Defined a future test plan.
- Updated [Governed Workflow Authoring Plan](../implementation-plans/governed-workflow-authoring-plan.md).
- Updated [Roadmap](../../ROADMAP.md).

## 3. Scope Explicitly Not Completed

This phase did not implement:

- workflow file writing;
- repository mutation;
- active workflow generation;
- workflow registration;
- workflow promotion or activation;
- workflow catalog storage;
- command execution;
- local check execution;
- provider calls;
- runtime state creation;
- approval decisions;
- report artifacts;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. Recommended Future Boundary

The plan recommends a future explicit command shape:

```sh
workflow-os author workflow \
  --from-recommendation <id> \
  --output workflows/drafts/<workflow-id>.workflow.yml
```

The generated file must be an inactive draft. It must not become registered or executable simply because it exists on disk.

The current dry-run command remains valid:

```sh
workflow-os author workflow --from-recommendation <id> --dry-run
```

## 5. Safety Boundary Summary

Future file output must:

- require an explicit source recommendation id;
- require an explicit relative output path;
- reject absolute paths and traversal;
- reject unsupported extensions;
- reject existing output files by default;
- reject duplicate workflow ids;
- use only bounded proposal vocabulary;
- keep owner and escalation as explicit obligations unless safely supplied later;
- keep side-effect posture unsupported/skipped/none unless explicitly reviewed later;
- keep report and handoff obligations visible;
- keep generated drafts inactive.

## 6. Privacy And Redaction Summary

The plan forbids generated draft content from copying:

- raw source contents;
- raw package script command bodies;
- raw dependency values;
- raw CI logs;
- provider payloads;
- issue or pull request bodies;
- parser payloads;
- absolute private paths;
- environment values;
- credentials, authorization headers, private keys, token-like strings;
- existing agent instruction bodies.

The future implementation must avoid embedding current user names, emails, machine names, home directories, and temp paths.

## 7. Test Coverage Planned

The future implementation test plan covers:

- current dry-run compatibility;
- required output path behavior;
- path rejection for absolute, traversal, and unsupported extensions;
- no overwrite;
- recommendation id validation and non-leakage;
- workflow id validation and conflict detection;
- inactive lifecycle;
- required authoring obligations;
- side-effect and report/handoff obligations;
- no runtime state;
- no command execution;
- no provider calls;
- no raw source, script, dependency, or secret copying;
- existing CLI, validation, scaffold, runtime, and docs regression coverage.

## 8. Governed Dogfood Summary

- Workflow: `dg/d`.
- Run ID: `run-1783400371383890000-2`.
- Approval ID: `approval/run-1783400371383890000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations; terminal status `Completed`.
- Scope approved: create a planning document for a future explicit inactive workflow draft file-output path.
- Strict non-goals: no implementation, workflow file writes, registration, promotion, command execution, provider calls, runtime state, schemas, examples, hosted behavior, writes, or release posture changes.
- Out-of-kernel work disclosed: file editing, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 9. Validation Commands Run

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783400371383890000-2 --phase planning`: passed.

## 10. Remaining Known Limitations

- File-output behavior is not implemented.
- Draft workflow rendering is not implemented.
- Workflow id selection remains open.
- Draft directory policy remains open.
- Promotion and activation remain future.
- Catalog/storage integration remains future.

## 11. Recommended Next Phase

Recommended next phase: governed workflow authoring file-output plan review.

The plan introduces the first repository mutation boundary in the authoring lane. It should be reviewed before any implementation creates draft workflow files.
