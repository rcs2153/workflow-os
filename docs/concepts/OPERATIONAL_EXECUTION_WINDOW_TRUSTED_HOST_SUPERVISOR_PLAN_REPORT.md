# Operational Execution Window And Trusted-Host Supervisor Plan Report

## 1. Executive Summary

The next P0 runtime-composition boundary is planned. The plan first adds an
atomic operational execution-window opening operation, then one injected
one-shot local trusted-host supervisor over the accepted opening capability.
Planning does not implement either boundary.

## 2. Scope Completed

- Defined the missing initial-window opening transaction.
- Bound opening to current source-backed authority, immutable run context,
  gate and policy commitments, trusted time, and an exact runtime cursor.
- Required window creation, first-attempt allocation, operation receipt,
  runtime event, and snapshot projection to commit atomically.
- Defined the one-shot injected trusted-host supervisor boundary.
- Defined first dispatch, turn-boundary yield, lawful resume, bounded outcome,
  reconciliation, concurrency, privacy, and backend posture.
- Split implementation so operational opening receives focused security review
  before host invocation is introduced.

## 3. Scope Explicitly Not Completed

No store API, SQLite operation, runtime event, supervisor, scheduler, executor
redispatch, automatic approval, provider mutation, OpenShell execution, nested
harness runtime, CLI, schema, hosted runtime, lineage, or release change is
implemented.

## 4. Key Planning Decision

The supervisor cannot open or reconstruct authority. Core must first commit an
operational window and started attempt and return a private one-use capability.
The supervisor may invoke one injected executor only with that capability and
must durably report the result back to Core.

## 5. Runtime Semantics

An executor turn boundary is a yield, not workflow completion or an invented
wait. Only a real terminal runtime event may terminate the run. The first
supervisor is one-shot and local; repeated scheduling remains a separate
future phase.

## 6. Privacy Posture

The boundary is limited to validated IDs, enums, revisions, timestamps,
counts, hashes, and commitments. Prompts, transcripts, source, specs, command
output, provider payloads, paths, environment values, credentials, and
reconstructable authority remain forbidden.

## 7. Governed Planning Record

- workflow: `dg/d`;
- run: `run-1790833556869438000-2`;
- approval: `approval/run-1790833556869438000-2/planning-approved`;
- presentation: `presentation/b36efc487c26f721`;
- presentation hash:
  `b36efc487c26f72120fd11d31e419b2aba9503b4c86e2c91c1ce611d47d10ac9`;
- approval outcome: granted under standing delegated-maintainer authority
  after the complete persisted planning handoff was presented;
- governed status: completed with 39 events, 1 approval, 0 retries, and 0
  escalations; and
- out-of-kernel work: repository inspection and plan authoring were performed
  by the external executor. The kernel did not edit files, execute checks,
  open a window, schedule an executor, mutate a provider, or create a model
  turn.

## 8. Validation

- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- governed phase close: passed with proof-enforced approval presentation and
  terminal `Completed` status.

## 9. Remaining Limitations

- Operational initial-window opening is not implemented.
- No host supervisor consumes continuity state.
- No scheduler can create or resume a model turn.
- SQLite is the only eligible continuity backend for the future proof.
- Filesystem and PostgreSQL remain unsupported.

## 10. Recommended Next Phase

Perform focused maintainer/security review of the plan. If accepted, implement
the atomic operational opening contract and SQLite projection before adding
the injected supervisor.
