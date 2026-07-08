# Governed Workflow Authoring Draft Status Implementation Review

## 1. Executive Verdict

Phase accepted; proceed to archive command planning.

The implementation satisfies the planned first draft cleanup/supersession slice:
`workflow-os author workflow draft-status --draft ...` provides bounded,
non-mutating status visibility for preserved inactive drafts without adding
archive/delete behavior, persisted approvals, catalog state, runtime state,
schemas, examples, provider calls, writes, or release posture changes.

## 2. Scope Verification

The phase stayed within the approved non-mutating inspection scope.

Confirmed no accidental:

- archive command;
- draft deletion;
- draft movement;
- draft metadata mutation;
- automatic cleanup after promotion;
- automatic promotion;
- workflow registration;
- persisted steward approval records;
- workflow catalog persistence;
- runtime state creation;
- workflow run creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- schema changes;
- examples;
- hosted or distributed behavior;
- external writes;
- release posture changes.

## 3. CLI Surface Assessment

The implemented command is:

```sh
workflow-os author workflow draft-status \
  --draft workflows/drafts/<name>.workflow.yml
```

This is an appropriate first surface because it makes preserved drafts easier to
understand after promotion while avoiding destructive cleanup behavior. The
command is additive and consistent with the existing authoring command family:
draft output, preflight, steward review, promote, and now status inspection.

The help text correctly describes the command as inspection-only and states
that it writes no files, promotes nothing, and registers nothing.

## 4. Status Semantics Assessment

The implementation derives three statuses from current file placement and active
workflow identity:

- `active_candidate` when no matching active workflow exists;
- `promoted_preserved` when the derived active workflow path exists and carries
  the same workflow id as the draft;
- `superseded_by_active` when the candidate workflow id or derived active path
  is already represented by active workflow state in a conflicting way.

This is appropriately conservative. It avoids claiming `stale_candidate`
without persisted review, approval, or catalog state that could prove staleness.
That limitation is documented in the implementation report and should remain
deferred.

## 5. Validation Boundary Assessment

The command reuses existing authoring path validation, which rejects unsafe
draft paths, unexpected directories, invalid filenames, invalid extensions, and
secret-like filename material.

The command fails closed for:

- missing Workflow OS project;
- invalid project validation;
- unsafe draft path;
- missing draft file;
- draft parse failure;
- active workflow read failure;
- active workflow parse failure.

Validation errors use stable codes. The implementation does not echo raw YAML or
secret-like values in failure paths covered by tests.

## 6. Non-Mutation Assessment

The command only reads project state and files required to classify one draft.
It does not write files, move drafts, delete drafts, archive drafts, promote
workflows, register workflows, create runtime state, write report artifacts,
execute commands, call providers, or touch a `StateBackend`.

Text and JSON output both include explicit boundary booleans, including:

- `files_written: false`;
- `active_workflow_file_written: false`;
- `draft_moved: false`;
- `draft_deleted: false`;
- `draft_archived: false`;
- `workflow_registered: false`;
- `workflow_promoted: false`;
- `runtime_state_created: false`;
- `report_artifact_written: false`.

## 7. Privacy And Redaction Assessment

The output is bounded to:

- relative draft path;
- relative active workflow path;
- candidate workflow id;
- content hash;
- status code;
- conflict status code;
- next-action code;
- boundary booleans.

The command does not copy raw draft YAML, active workflow YAML, source contents,
package script bodies, dependency values, lockfile contents, CI logs, command
output, provider payloads, parser payloads, absolute private paths, environment
values, credentials, authorization headers, private keys, token-like strings, or
steward review reason text.

JSON output uses the existing CLI JSON escaping helper and remains preview
surface, consistent with neighboring authoring commands.

## 8. Test Quality Assessment

Focused CLI tests cover:

- unpromoted draft reports `active_candidate`;
- promoted and preserved draft reports `promoted_preserved`;
- JSON output is bounded and non-mutating;
- missing draft fails closed;
- unsafe/secret-like path fails closed without leakage;
- no active workflow file is written by status inspection;
- no runtime state is created by status inspection.

The full workspace suite also covers existing authoring, promotion,
steward-review, WorkReport, EvidenceReference, Diagnostic, validation, adapter
telemetry, and runtime behavior.

Non-blocking test gap: a direct `superseded_by_active` fixture would improve
coverage for the conflict branch. The branch is simple and bounded, but it is
worth adding before archive/delete work.

## 9. Documentation Review

Documentation now says:

- the draft cleanup/supersession plan has an implemented first `draft-status`
  slice;
- the command is non-mutating;
- archive/delete cleanup remains unimplemented;
- automatic cleanup remains unimplemented;
- persisted steward approvals remain unimplemented;
- workflow catalog state remains unimplemented;
- runtime state, command execution, provider calls, schemas, examples, hosted
  behavior, writes, and release posture changes remain unimplemented.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add a direct `superseded_by_active` CLI test fixture.
- Consider whether active workflow parse failure should include a separate
  status output before failing, if future operators need partial diagnostics.
- Plan archive behavior separately before deletion.
- Plan persisted approval/catalog state before claiming stale-draft detection.

## 12. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 13. Governed Dogfood Summary

- Workflow: `dg/review`.
- Run ID: `run-1783480767200199000-2`.
- Approval ID: `approval/run-1783480767200199000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: create maintainer review for draft-status implementation and
  validate scope.
- Strict non-goals: no implementation changes, mutation, archive, delete,
  runtime state, schemas, examples, or writes.

Phase close: completed.

Event summary: 39 events; 1 approval; 0 retries; 0 escalations.

Out-of-kernel work: review documentation edits, roadmap/planning doc updates,
validation commands, git/PR actions, and GitHub merge actions were performed
outside the kernel and are disclosed here.

## 14. Recommended Next Phase

Recommended next phase: archive command planning.

Reason: status inspection now gives maintainers a safe way to identify preserved
and superseded drafts. The next decision should be the archive policy and CLI
shape, still before any deletion behavior or catalog persistence.
