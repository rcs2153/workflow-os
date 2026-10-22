# Trusted-Host Cargo Format Workflow Composition Implementation Report

## 1. Executive Summary

The accepted trusted-host Cargo format workflow composition plan is
implemented. One unpublished local application path now executes exactly
three ordered no-write checks: Workflow OS project validation, documentation
validation, and `cargo fmt --all --check`. Core owns topology, progression,
profile selection, durable dispatch identity, terminal truth, and report
composition. The host receives opaque prepared operations and may invoke at
most three of them.

The implementation remains local, private, additive, and no-write. It does
not generalize the trusted-host registry or authorize another command,
provider, mutation, public CLI surface, schema, hosted path, or release change.

## 2. Scope Completed

- Added an exact closed registry for the accepted project-validation,
  docs-check, and Cargo-format profiles.
- Added an exact three-step admission request and Core-owned admission path.
- Bound all three command contracts into the immutable run bundle.
- Derived current step and profile from durable state rather than caller
  memory.
- Added exact topology and causal-history validation for three ordered steps.
- Added an unpublished `three-step-no-write` local-host mode with a maximum
  three-operation loop.
- Added deterministic v4 WorkReport composition and create-only artifact
  persistence.
- Preserved existing one-step and two-step entry points and tests.

## 3. Scope Explicitly Not Completed

This phase did not add clippy, tests, builds, generic Cargo execution,
source-writing format mode, arbitrary profile registration, caller-authored
ordering, providers, network authority, credentials, SideEffect execution,
external writes, automatic approval, schedulers, queues, public CLI behavior,
schemas, SDK changes, onboarding examples, hosted parity, parallelism, nested
harnesses, or release-posture changes.

## 4. Registry And Admission Boundary

`TrustedHostThreeStepNoWriteProfileRegistry` accepts exactly the three reviewed
profiles and validates exact skill/profile identity, command inventory, and
explicit-zero authority posture. `TrustedHostThreeStepNoWriteAdmissionRequest`
contains run identity and observation context but no caller-selected step,
profile, command, successor, result, or report identity.

Fresh admission validates the exact workflow and complete immutable bundle.
Re-admission loads the stored bundle and derives the only lawful current step
from causal durable history. The local host resolves explicit tool paths and
roots, but Core decides what operation those profiles may perform.

## 5. Topology And Progression

The only accepted sequence is:

```text
project-validation -> docs-check -> cargo-format-check -> terminal
```

Each passing non-terminal result atomically commits its result, invocation
terminal, and exact successor schedule. A failed or timed-out result commits
terminal failure without a successor. The host cannot schedule a step, append
lifecycle events, patch a snapshot, or infer progression from process memory.

Restart after the first or second committed result resumes only the remaining
work. Terminal replay invokes no process and appends no event.

## 6. Dispatch And Execution Identity

Every attempted step requires exactly one matching authorized dispatch. The
dispatch contains the durable executor commitment produced from the resolved
closed profile at admission. Report composition recomputes the invocation
commitment from the immutable step input and that durable executor commitment,
then rejects missing, duplicate, reordered, or substituted bindings.

For a pre-result identity failure, the durable dispatch commitment is the
authoritative admitted execution identity. Report composition does not claim
that a subsequently mutated executable or protected source state can reproduce
that identity after failure.

## 7. Pre-Result Failure Truth

The implementation accepts one narrow pre-result terminal shape:

```text
step scheduled < dispatch admitted < invocation failed < run failed
```

No `LocalCheckResultRecorded` event is fabricated. The v4 report binds the
schedule, durable dispatch, failed invocation, and terminal event; states that
no bounded result was accepted; and marks later checks not attempted. It does
not infer an unpersisted formatter, source, or toolchain failure cause.

## 8. Terminal Report And Artifact

The new report contract is
`trusted-host-three-step-terminal-no-write@v4`. Its commitment covers the run,
immutable bundle, exact ordered topology, closed profile and command identity,
durable dispatch event and executor commitment, accepted local-check results,
invocation terminals, and terminal run event. Event citations are emitted in
causal dispatch, optional result, invocation-terminal, and run-terminal order.

Artifact persistence is create-only. Exact duplicates reconcile without
rewriting; conflicting or unreadable artifacts fail closed. Report or artifact
failure cannot rewrite terminal workflow truth.

## 9. Privacy And Error Posture

Stable error families do not copy source or toolchain paths, process output,
command transcripts, source/spec/parser/provider payloads, environment values,
credentials, or caller-authored prose. Debug surfaces redact profile details,
roots, commitments, and runtime identities. Reports cite bounded references
and do not create `EvidenceReference` values implicitly.

The profile proves bounded identity and final-state equivalence. It does not
claim operating-system sandboxing, network isolation, transient-write
prevention, or independent third-party toolchain attestation.

## 10. Test Coverage

Focused tests prove exact successful order; failure at each step; no successor
after failure; caller step selection rejection before state creation; restart
after first and second commits; exact terminal replay; v4 artifact creation;
stable-ID citations; durable dispatch binding; third-step non-zero failure;
pre-result identity failure without a fake result; no later check after early
failure; and existing one-step/two-step compatibility.

The full workspace suite covers existing executor, approval, evidence,
reporting, provider, state, hosted, and runtime behavior. The explicit live
smoke exercises the reviewed bundled stable Cargo/cargo-fmt/rustfmt chain
without installing dependencies or enabling network access.

## 11. Commands Run And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-local-host --test foreground_admission`: passed,
  35 tests.
- `cargo test --workspace`: passed.
- opt-in real Cargo format smoke against the bundled stable toolchain: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed through `npm run check`.
- `git diff --check`: passed.

## 12. Governed Implementation Record

- Workflow ID: `dg/runtime-composition`
- Run ID: `run-1792682059492664000-2`
- Approval ID:
  `approval/run-1792682059492664000-2/composition-approved`
- Presentation ID: `presentation/e9ee5ed7edd9f9cd`
- Presented content hash:
  `e9ee5ed7edd9f9cd0407e9fb99a129013b273bd74d6764743e5e40d09a4840e5`
- Approval outcome: granted under delegated-maintainer authority for the exact
  reviewed three-step no-write composition scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations, one
  persisted presentation record, and a present approval-event proof marker
- Out-of-kernel work: the delegated maintainer edited source, tests, and docs;
  ran processes and validation; and will perform git/PR operations. The kernel
  governed scope and approval but did not perform those actions.

## 13. Remaining Known Limitations

- No operating-system process, filesystem, or network sandbox is added.
- Final-state equivalence cannot observe a transient write restored before the
  post-execution commitment.
- The executable/toolchain identity is self-committed, not independently
  attested.
- The exact path is unpublished and supports only the three reviewed checks.
- Runtime report/artifact rendering remains private to the local application
  path.

## 14. Recommended Next Phase

Perform a focused maintainer/security review of this implementation. Do not
broaden the registry, add clippy/tests/build profiles, enable providers or
writes, expose a public surface, or expand topology before that review.
