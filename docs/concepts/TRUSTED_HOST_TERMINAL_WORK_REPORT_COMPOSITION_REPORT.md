# Trusted-Host Terminal WorkReport Composition Report

## 1. Executive Summary

The unpublished trusted-host application now composes one validated in-memory
terminal `WorkReport` for the closed docs-check and project-validation profiles.
Core records one payload-free `LocalCheckResultRecorded` event in the same
SQLite transaction as invocation and run terminalization, rehydrates the exact
terminal run, validates the immutable operation binding, and constructs the
report through existing `WorkReport` constructors.

Exact terminal replay uses the durable result event. It does not rerun the
process or append workflow events. Report composition failure remains separate
from workflow terminal truth.

## 2. Scope Completed

- Added a payload-free durable event for passed, failed, and timed-out concrete
  local-check outcomes.
- Bound the event to command, profile, step, invocation, attempt, immutable run
  bundle, and invocation-terminal event identity.
- Included the event in the existing atomic trusted-host terminal lifecycle
  transaction and its request commitment.
- Preserved the same-call concrete result through the closed-profile executor.
- Added a private Core composition function that derives report identity,
  contract identity, timestamp, actor, citations, limitations, and handoff text.
- Composed reports only after terminal commit and exact run rehydration.
- Reconstructed the same report posture during exact terminal replay without
  process execution or event mutation.
- Exposed fixed generated/failed posture from the unpublished local host without
  rendering report contents or errors.

## 3. Scope Explicitly Not Completed

This phase did not add:

- report persistence or a `WorkReportArtifact`;
- released CLI report display, JSON export, or filesystem output;
- automatic reports for other executor paths;
- caller-selected report contracts, summaries, citations, or metadata;
- another local-check profile, arbitrary commands, scheduling, or default
  activation;
- provider access, credentials, network access, SideEffects, or writes;
- automatic approval or policy broadening;
- hosted parity, OpenShell integration, nested harnesses, or reasoning lineage;
- workflow schema, examples, or release-posture changes.

## 4. Result Event And Atomicity

`LocalCheckResultRecorded` stores stable references and closed vocabulary only.
It contains no stdout, stderr, summaries, paths, environment values, parser
payloads, source contents, or provider payloads. Only concrete `Passed`,
`Failed`, and `TimedOut` states are valid; constructor and deserialization paths
reject all other result states with a stable non-leaking error.

For concrete results, SQLite commits the continuity projection, result event,
invocation terminal event, and run terminal event in one lifecycle batch. The
same event is included in the operation request commitment, so conflicting
replay cannot substitute a different result binding.

## 5. Report Composition And Replay

The private composition boundary verifies:

- terminal run posture;
- exact immutable bundle identity and version;
- exactly one result-binding event;
- exact profile, command, step, and bundle binding;
- one matching invocation-terminal event; and
- a later matching run-terminal event.

It derives a deterministic report ID from the run, bundle root, closed profile,
step, command, and result reference. The fixed private contract is
`trusted-host-terminal-no-write` version `v1`. The report cites the result,
result event, invocation terminal event, and run terminal event by stable
reference. Missing, duplicate, or conflicting durable bindings fail closed.

The foreground host prints only fixed generated/failed posture. On terminal
replay, it composes from rehydrated state and leaves event count unchanged.

## 6. Workflow Semantics

- Passed checks retain completed workflow truth.
- Failed and timed-out concrete checks retain failed workflow truth.
- Report generation happens after terminal state is durable.
- Report failure does not append a replacement terminal event, rerun the check,
  or rewrite workflow status.
- Waiting, blocked, prepared, and denied outcomes do not produce a terminal
  report.

## 7. Privacy And Redaction

The durable result event and report contain references and fixed bounded text,
not command output or project contents. Debug implementations redact binding
identities. Host output and errors use fixed stable posture and codes. Invalid
serialized result states fail without echoing the supplied value.

Reports remain `Internal` despite the read-only, no-write execution posture.

## 8. Test Coverage

Focused coverage proves:

- completed project-validation and docs-check runs generate report posture;
- failed checks persist one `Failed` result binding and generate a terminal
  failed-run report posture;
- successful checks persist one `Passed` result binding;
- terminal replay appends no events;
- the result event is ordered before invocation and run terminal events;
- exact immutable bundle binding is preserved;
- invalid non-concrete serialized result status fails closed without leakage;
- fixed host output excludes command output and private error details; and
- existing WorkReport, runtime-event, audit-projection, Core, CLI, hosted, and
  workspace behavior remains green.

## 9. Commands Run And Results

- `cargo fmt --all --check`: passed after formatting.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; ordinary ignored live-provider tests remain
  intentionally opt-in.
- `cargo test -p workflow-core --test work_report`: passed, 227 tests.
- `cargo test -p workflow-local-host --test foreground_admission`: passed, 13
  tests.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Remaining Known Limitations

- The generated report exists only in memory and is not available after process
  exit except by deterministic recomposition from durable terminal state.
- The host exposes fixed report posture only; report contents have no released
  user surface.
- Only the two closed trusted-host no-write profiles participate.
- Internal handler failure without a concrete check result creates no result
  citation.
- Full workflow-declared `WorkReportContract` enforcement remains deferred.
- The dedicated draft-pull-request live sandbox proof remains separately
  dependency-blocked by its sandbox credential.

## 11. Governed Phase Record

- Dogfood workflow ID: `dg/runtime-composition`
- Run ID: `run-1792664236236479000-2`
- Approval ID:
  `approval/run-1792664236236479000-2/composition-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/12aa5557ef539f6d`
- Terminal status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Validation summary: formatting, clippy, workspace tests, focused report and
  host tests, docs validation, and diff hygiene passed

Repository edits, shell commands, validation commands, and report composition
were executed outside the kernel by the agent under the approved phase scope.
The kernel coordinated governance and recorded approval/phase history; it did
not edit files, execute Cargo or npm, create commits, push branches, or open a
pull request. No check, handler, report, git, or provider activity was
simulated.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of the trusted-host terminal
WorkReport composition implementation.

Do not broaden into report artifacts, persistence, released CLI rendering,
another profile, arbitrary commands, scheduling, providers, writes, hosted
expansion, or nested harness execution during that review.
