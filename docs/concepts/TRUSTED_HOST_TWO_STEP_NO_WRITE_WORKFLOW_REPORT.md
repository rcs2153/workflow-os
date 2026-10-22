# Trusted-Host Two-Step No-Write Workflow Report

## 1. Executive Summary

The first exact two-step trusted-host workflow is implemented behind the
unpublished local host boundary. It runs Workflow OS project validation and
then the docs check, records both authoritative local-check results, produces
one private v3 terminal `WorkReport`, and persists one SQLite report artifact.

Core owns the immutable topology, current runnable step, matching closed
profile, and durable progression. The host can request the exact mode and run
the opaque prepared operations, but it cannot select a step, substitute a
profile, introduce a command, or mutate workflow history.

## 2. Scope Completed

- Added one closed two-profile registry for the existing project-validation
  and docs-check profiles.
- Added one explicit two-step admission request without caller-selected step or
  profile fields.
- Admitted only the exact ordered two-step workflow and immutable bundle.
- Committed first-step result, invocation terminal event, and second-step
  scheduling atomically.
- Re-admitted the second step from durable history after a process restart.
- Terminalized only after the second step, or on the first failing step.
- Added one bounded unpublished host loop with a maximum of two operations.
- Added one private v3 report and artifact path over the complete terminal
  history.

## 3. Scope Explicitly Not Completed

This phase did not add a third profile, arbitrary command execution, providers,
external writes, a public CLI surface, workflow schema changes, examples,
hosted execution, parallelism, branching, retries beyond existing closed
semantics, nested harnesses, or automatic execution.

## 4. Core Admission And Topology

`TrustedHostTwoStepNoWriteProfileRegistry` accepts only the two canonical
resolved profiles and validates their explicit-zero authority posture.
`TrustedHostTwoStepNoWriteAdmissionRequest` carries execution, bundle, and
trusted-time inputs but no step or profile choice.

Fresh admission validates the exact workflow topology and builds one immutable
bundle containing both handler references and command contracts. Re-admission
loads that bundle and derives the current step and profile from authoritative
event history. Unsupported topology, identity substitution, invalid history,
or profile mismatch fails with stable payload-free posture.

## 5. Atomic Progression And Workflow Truth

The trusted-host lifecycle can now express either terminal completion or an
exact next-step schedule. SQLite commits the local-check result, invocation
terminal event, and the selected completion event in one transaction. A
successful first step therefore appends `StepScheduled` for `docs-check`
without appending `RunCompleted`. Failure terminalizes immediately and never
schedules the next step.

The transaction remains the progression authority. The foreground process is
not trusted to patch state, choose a successor, or infer completion.

## 6. Restart And Replay Behavior

A fresh process can re-admit a run after the first atomic commit. Core validates
the exact result/schedule prefix, derives `docs-check`, and prepares only that
operation. Terminal replay returns the existing terminal posture, does not
rerun either check, and does not append workflow events. Missing exact artifacts
can be recovered through the existing create-only reconciliation path.

## 7. Terminal Report And Artifact

The two-step path uses the distinct
`workflow-os/trusted-host-terminal-report/v3` commitment and contract identity
`trusted-host-two-step-terminal-no-write` version `v3`. The report validates
ordered result, invocation, bundle, profile, command, step, and terminal-event
bindings before citing the stable local-check result and workflow event IDs.

A first-step failure produces one result citation and explicitly discloses that
the docs check was not attempted. A second-step failure cites both results. The
artifact path preserves existing generation-failure, create-only write,
duplicate reconciliation, and non-overwrite behavior.

## 8. Privacy And Error Posture

The new public-hidden request and registry have bounded redaction-safe `Debug`
implementations. Errors use stable codes and fixed messages. Reports cite
stable references rather than command output. Host output remains fixed posture
text and does not expose paths, process output, source/spec contents, parser
payloads, environment values, credentials, authorization headers, keys, or
tokens.

## 9. Test Coverage

Focused coverage proves:

- ordered two-step success with all required report sections and one v3
  artifact;
- project-validation failure without docs scheduling;
- docs-check failure with both durable results;
- restart after the first atomic commit and second-step-only continuation;
- terminal replay without event mutation;
- caller-selected step rejection before state creation;
- stable-ID report citations and honest not-attempted disclosure;
- artifact recovery and duplicate reconciliation; and
- unchanged existing one-step behavior.

The focused foreground admission suite contains 26 passing tests.

## 10. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-local-host --test foreground_admission`: passed, 26
  tests.
- `cargo test --workspace`: passed; explicitly opt-in live-provider tests
  remained ignored by design.
- `npm run check`: passed, including docs, dogfood helper, integration-helper,
  TypeScript SDK, and contract checks.
- `npm run check:integrations`: passed.
- `git diff --check`: passed.

## 11. Governed Phase Record

- Dogfood workflow ID: `dg/runtime-composition`
- Run ID: `run-1792668984875645000-2`
- Approval ID: `approval/run-1792668984875645000-2/composition-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/b0dc3fc332ee5edb`
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.

The kernel governed the approved runtime-composition scope. Repository
inspection, code edits, command execution, tests, documentation, and git work
were performed by the agent outside the kernel under that bounded approval.

## 12. Remaining Limitations

- The mode remains unpublished and private to the local host crate.
- The registry contains exactly two closed profiles.
- The host still performs the process invocation after Core prepares an opaque
  operation.
- Reports and artifacts have no released inspection or export surface.
- No provider, mutation, general scheduler, parallel, or nested-harness
  capability is implied.

## 13. Recommended Next Phase

Perform focused maintainer/security review of this implementation. The review
should prioritize atomic progression, history validation, restart behavior,
profile/step non-substitution, v3 report integrity, failure honesty, and
one-step compatibility. Any blocker should be fixed before the trusted-host
runtime is broadened.
