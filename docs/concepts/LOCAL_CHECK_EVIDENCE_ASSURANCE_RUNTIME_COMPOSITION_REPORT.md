# Local Check Evidence Assurance Runtime Composition Report

## Summary

Workflow OS now composes the reviewed weak local-check evidence-assurance
projection into the existing closed trusted-host project-validation result
path.

Core constructs the projection beside the exact process request it dispatches.
The projection travels with the structured local-check result through
attestation evaluation, governance contribution, reassessment, current-runtime
fact bridging, and every authoritative route result. No caller-created or
deserialized projection enters this path.

## Implemented Boundary

The same-call construction binds:

- the stored immutable run bundle;
- workflow, run, and step identity;
- the immutable local-check execution binding;
- the selected handler commitment;
- a private commitment to the exact executable, arguments, working directory,
  sanitized environment, timeout, and executable content hash used for the
  dispatched request;
- the structured local-check result identity and status; and
- the current weak assurance posture.

The authoritative quiet, visible, approval-required, denied, and existing
terminal route results expose read-only assurance projections. Their fields
remain private, and the route dispatcher constructs them only from the
crate-private execution path.

## Current Assurance

The closed project-validation path returns:

- producer: `operator_selected_content_bound`;
- execution: `trusted_host_observed`;
- attestation: `unattested`; and
- external verifiability: `unavailable`.

The existing local-process attestation verifier still evaluates its own proof
for the governance contribution. This composition deliberately does not use
that proof to upgrade the public validator-assurance projection. A later phase
must separately review any stronger runtime projection.

## Retry And Replay Posture

An existing terminal route reexecutes the closed local check as part of its
fresh reassessment. That reassessment returns a fresh Core-owned projection:

- the immutable execution-binding and projection fingerprints change because
  the observation has a new creation time;
- the resolved process-boundary commitment remains stable when the exact
  executable and request remain unchanged; and
- the assurance posture remains weak and unattested.

No projection is persisted or reconstructed from serialized data in this
phase.

## Security Properties

- The resolved-execution commitment is computed from the exact request passed
  to the handler, not from a later reconstruction.
- Executable identity is available to the crate-private handler contract in all
  feature configurations needed by this path.
- Public route result fields are private and have no caller-supplied projection
  input.
- Debug output exposes counts and bounded posture, not commitments, paths,
  process payloads, or environment data.
- Minimum-assurance comparison remains classification only and is not used by
  a policy, approval, or execution gate.
- Workflow result, governance decision, approval, and report semantics are
  unchanged.

## Explicit Non-Goals

This phase did not add:

- projection persistence, events, replay reconstruction, or recovery state;
- WorkReport citations or report-body commitment;
- minimum-assurance policy enforcement;
- generic evidence producers or arbitrary command execution;
- CLI, schema, SDK, or hosted surfaces;
- registered producer identity, signatures, external witnesses, or remote
  attestation;
- provider mutation, nested harness execution, or publication; or
- storage-owner tamper resistance.

## Governed Phase

- Workflow: `dg/runtime-composition`
- Run: `run-1793854808751140000-2`
- Approval:
  `approval/run-1793854808751140000-2/composition-approved`
- Presentation: `presentation/96843c64fb13de76`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Kernel freshness: rebuilt from the exact checkout before phase start
- Repository edits, validation, and review ran outside the kernel under the
  approved execution scope

## Validation

- Focused fresh-route assurance projection test: passed.
- Focused terminal-reassessment projection test: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy -p workflow-core --all-targets --all-features -- -D warnings`:
  passed.
- `cargo test -p workflow-core --all-features`: passed, including 781 unit
  tests, 345 local-executor tests, the remaining integration suites, and doc
  tests; declared opt-in live and external-tool tests remained ignored.
- `git diff --check`: passed.

## Next Phase

Proceed only to the model-only versioned complete WorkReport body-commitment
phase after the focused review of this composition is accepted.
