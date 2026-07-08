# Workflow Catalog Steward Review Persistence Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds an explicit opt-in stewardship persistence path for
`author workflow steward-review` while preserving preview-only behavior by
default. It writes one validated local catalog stewardship record and does not
promote workflows, register workflows, persist approval records, create runtime
state, call providers, add schemas, update examples, enable writes, or change
release posture.

## 2. Scope Verification

The phase stayed within the approved persistence boundary.

Confirmed absent:

- no promotion catalog writes;
- no archive metadata writes;
- no workflow runtime registration;
- no catalog repair command;
- no automatic workflow generation;
- no draft deletion;
- no runtime state creation;
- no command execution;
- no provider calls;
- no hosted collaboration behavior;
- no workflow schema changes;
- no example updates;
- no write-capable adapters;
- no release posture changes.

## 3. CLI Boundary Assessment

The CLI remains preview-only unless `--persist-stewardship` is supplied.

Default invocation:

```sh
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision approved-for-promotion \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason>
```

continues to report `author_workflow_steward_review_preview`, discloses
`files_written: false`, and creates no catalog root.

Opt-in invocation:

```sh
workflow-os author workflow steward-review \
  --draft workflows/drafts/<name>.workflow.yml \
  --decision approved-for-promotion \
  --reviewer user/<reviewer> \
  --reason <bounded-review-reason> \
  --persist-stewardship
```

reports `author_workflow_steward_review_persisted`, writes exactly one
stewardship record, and discloses that workflow files, promotion, approval
persistence, commands, providers, and runtime state remain untouched.

`--catalog-root` is correctly rejected unless `--persist-stewardship` is also
present.

## 4. Persistence Boundary Assessment

The persistence helper constructs a `WorkflowStewardshipRecord` through the
reviewed model constructor and writes it through `LocalWorkflowCatalogStore`.
This preserves the existing validation and write-if-absent behavior.

The record captures bounded stewardship identity:

- decision id;
- decision kind;
- workflow id;
- draft path;
- candidate content hash;
- reviewer;
- bounded reason summary;
- strict non-goals;
- conservative sensitivity and empty redaction metadata.

It does not write workflow files, active workflow catalog records, archive
records, runtime events, state backend entries, approval records, report
artifacts, provider payloads, or command output.

## 5. Failure And Atomicity Assessment

Failure behavior is conservative:

- preflight blockers stop review before persistence;
- invalid reviewer and invalid/secret-like reason fail before persistence;
- unsafe catalog roots fail before persistence;
- invalid record construction maps to a stable bounded error;
- duplicate record write fails closed through `write_stewardship_record_if_absent`;
- duplicate persistence does not overwrite the existing record.

Error messages use stable codes and do not echo unsafe catalog roots,
secret-like values, raw workflow YAML, command output, provider payloads, or
private path material.

## 6. Privacy And Redaction Assessment

The implementation avoids copying raw workflow bodies, source contents, parser
payloads, command output, provider payloads, environment values, credentials, or
token-like values.

The persisted reason is a bounded review reason validated by the existing
steward-review/model path. CLI output and JSON output do not echo the reason.
Persisted records remain local catalog records and are not emitted as runtime
events or report artifacts.

## 7. Test Quality Assessment

The focused tests cover the important boundary:

- preview mode remains non-mutating and does not create `.workflow-os/catalog`;
- explicit `--persist-stewardship` writes one stewardship record;
- human output discloses the exact persistence boundary;
- JSON output discloses persisted posture without raw review text;
- `--catalog-root` requires `--persist-stewardship`;
- unsafe catalog root is rejected without leakage;
- duplicate persistence fails closed and preserves the original record;
- existing steward-review success, blocked, non-authorizing, JSON, and
  secret-like reason tests still pass.

The full workspace test suite passed during implementation validation.

## 8. Documentation Review

Documentation now states:

- opt-in steward-review persistence is implemented;
- default steward-review remains preview-only;
- `--persist-stewardship` writes one local catalog stewardship record;
- promotion catalog writes are not implemented;
- archive metadata writes are not implemented;
- runtime workflow registration is not implemented;
- catalog repair is not implemented;
- schemas, examples, hosted behavior, provider calls, write-capable adapters,
  and release posture changes remain deferred.

## 9. Blockers

No blockers.

## 10. Non-Blocking Follow-Ups

- Consider whether preview JSON should preserve the older root key spelling
  exactly until the next preview version bump. The JSON surface is explicitly
  preview-only, and tests continue to assert the important `mode` and boundary
  fields, so this is not a blocker.
- Consider adding a future read command for one stewardship decision id once
  promotion/archive integration needs direct citation lookup.

## 11. Validation

Implementation validation run before this review:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.

Review validation:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `npm run check:docs` passed.

Governed review run:

- workflow id: `dg/review`;
- run id: `run-1783531400719739000-2`;
- approval id: `approval/run-1783531400719739000-2/review-scope-approved`;
- approval outcome: granted;
- event summary:
  `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`,
  `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`,
  `RunValidated:1`, `SkillInvocationRequested:6`,
  `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`,
  `StepScheduled:6`;
- total events: 39;
- out-of-kernel work: repository documentation edits, validation commands,
  git/PR handling, and this review report were performed by the agent outside
  the kernel and disclosed here.

## 12. Recommended Next Phase

Recommended next phase: promotion catalog write planning.

Reason: steward-review can now produce a durable local decision reference. The
next planning phase should decide how active promotion should optionally cite or
require that stewardship decision before writing active workflow files and
eventually catalog records, while still avoiding runtime registration, schemas,
examples, provider calls, hosted behavior, and release posture changes.
