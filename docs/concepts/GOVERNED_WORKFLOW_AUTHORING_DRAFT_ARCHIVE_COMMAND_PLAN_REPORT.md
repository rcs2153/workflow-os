# Governed Workflow Authoring Draft Archive Command Plan Report

## 1. Executive Summary

This planning phase defines the next bounded governed workflow authoring cleanup
step: an explicit local draft archive command.

The planned command should move one eligible inactive draft from
`workflows/drafts/` into `workflows/drafts/archive/` without deleting it,
touching active workflow files, registering workflows, creating runtime state,
executing commands, calling providers, writing report artifacts, adding schemas,
or changing release posture.

## 2. Scope Completed

- Created
  [Governed Workflow Authoring Draft Archive Command Plan](../implementation-plans/governed-workflow-authoring-draft-archive-command-plan.md).
- Defined proposed CLI shape for `workflow-os author workflow archive-draft`.
- Chose `workflows/drafts/archive/` as the first archive surface.
- Defined first eligibility as `promoted_preserved` and
  `superseded_by_active`.
- Deferred archiving `active_candidate` drafts.
- Defined reviewer/reason requirements.
- Defined bounded output, privacy, error-handling, and test expectations.
- Updated roadmap/planning docs to link the archive plan.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- archive command behavior;
- draft deletion;
- automatic cleanup after promotion;
- automatic archiving;
- active workflow mutation;
- workflow registration changes;
- persisted steward approval records;
- workflow catalog state;
- runtime state;
- workflow runs;
- command execution;
- provider calls;
- report artifacts;
- schemas;
- examples;
- hosted or distributed behavior;
- write-capable adapters or external writes;
- release posture changes.

## 4. Plan Summary

The plan recommends:

```sh
workflow-os author workflow archive-draft \
  --draft workflows/drafts/<name>.workflow.yml \
  --reviewer user/<reviewer> \
  --reason <bounded-reason>
```

It also recommends `--dry-run` so the archive decision can be previewed without
moving files.

The command should reuse existing draft path validation and draft-status
classification, refuse unsafe or ambiguous cases, and emit bounded text/JSON
output if JSON is added.

## 5. Safety Boundary Summary

The planned command should:

- validate the project first;
- validate the draft path;
- derive draft status before archive;
- refuse unpromoted active candidates by default;
- refuse archive destination overwrite;
- move exactly one draft file;
- leave active workflow files untouched;
- create no runtime state;
- write no report artifacts;
- execute no commands;
- call no providers.

## 6. Privacy And Redaction Summary

The plan forbids copying raw draft YAML, active workflow YAML, source contents,
manifest bodies, package scripts, dependency values, lockfiles, CI logs, command
output, provider payloads, parser payloads, absolute private paths, environment
values, credentials, authorization headers, private keys, token-like strings,
reviewer reason text, or existing agent instruction bodies into output or
errors.

Allowed output is limited to bounded relative paths, workflow ids, content
hashes, status codes, action codes, and boundary flags.

## 7. Test Coverage Planned

The future implementation should test:

- dry-run non-mutation;
- promoted-preserved archive;
- superseded archive;
- active workflow preservation;
- project validation after archive;
- active candidate rejection;
- destination overwrite rejection;
- missing and unsafe draft failure;
- secret-like reason rejection;
- raw YAML and reason non-leakage;
- no runtime state, report artifacts, command execution, or provider calls;
- bounded JSON if added;
- existing authoring regression coverage.

## 8. Commands Run And Results

- `npm run check:docs` - passed.

## 9. Governed Dogfood Summary

- Workflow: `dg/d`.
- Run ID: `run-1783481737974407000-2`.
- Approval ID: `approval/run-1783481737974407000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: archive command planning only.
- Strict non-goals: no archive implementation, delete behavior, automatic
  cleanup, persisted approvals, catalog state, runtime state, schemas,
  examples, providers, writes, or release posture changes.

Phase close: completed.

Event summary: 39 events; 1 approval; 0 retries; 0 escalations.

Out-of-kernel work: planning documentation edits, roadmap/planning doc updates,
validation commands, git/PR actions, and GitHub merge actions were performed
outside the kernel and are disclosed here.

## 10. Remaining Known Limitations

- The archive command is not implemented.
- Archive destination collision handling is planned but not implemented.
- Archive of unpromoted drafts remains deferred.
- Persisted approval/catalog semantics remain deferred.
- Runtime event/audit projection for authoring archive actions remains
  deferred.
- Deletion remains explicitly out of scope.

## 11. Recommended Next Phase

Recommended next phase: draft archive command implementation.

Reason: draft-status has been accepted, and the archive command is the next
small non-destructive cleanup action. It should come before any delete behavior,
automatic cleanup, persisted approval consumption, or catalog state.
