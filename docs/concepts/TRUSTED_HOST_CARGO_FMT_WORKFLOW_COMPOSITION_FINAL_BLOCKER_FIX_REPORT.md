# Trusted-Host Cargo Format Workflow Composition Final Blocker Fix Report

## 1. Executive Summary

The remaining trusted-host three-step composition blockers are fixed. Exact
history validation now accepts a bounded pre-result integrity failure on the
last scheduled step rather than only on the final declared workflow step.
Focused regressions prove first- and second-step pre-result failure, duplicate
dispatch rejection, and composed Cargo timeout truth through the v4 terminal
report artifact.

The fix remains private, local, explicit, and no-write. It does not add a
profile, topology, provider, public CLI behavior, schema, hosted execution,
nested harness execution, or release-posture change.

## 2. Blockers Fixed

This phase closes the three blockers from the first blocker-fix review:

1. pre-result failure is validated against the exact scheduled prefix;
2. duplicate dispatch commitments are rejected directly by the three-step
   terminal replay boundary; and
3. a composed Cargo timeout records `TimedOut`, fails terminally, schedules no
   additional work, and produces one bounded v4 artifact.

## 3. Implementation Approach

The production change is one boundary correction in the existing private
history validator. The validator computes the durable scheduled-step count
once and uses it for both the in-progress invocation gap and the terminal
pre-result failure path. A pre-result failure is therefore valid only when it
belongs to the last step actually scheduled for that run.

The validator still requires the exact deterministic invocation request,
matching authorized dispatch, one matching failed invocation, and terminal
`RunFailed` event. It does not fabricate a local-check result or infer an
unpersisted failure cause.

## 4. Early Pre-Result Failure Proof

Focused tests now admit the first or second step, replace the already-bound
executable after dispatch admission, and execute through the trusted-host
supervisor. Each run:

- fails terminally;
- contains only the scheduled prefix;
- contains results only for previously completed steps;
- records the matching failed invocation for the attempted step;
- composes and persists one bounded v4 artifact; and
- discloses the missing attempted-step result without inventing evidence.

The existing third-step restart and pre-result identity-substitution proof is
preserved.

## 5. Duplicate Dispatch Proof

A direct corruption regression duplicates one durable dispatch invocation
commitment inside an otherwise completed three-step run. Terminal replay
fails closed before artifact recovery. No replacement artifact is persisted,
and the bounded error does not expose repository paths or the mutated payload.

## 6. Timeout And Artifact Proof

The composed timeout regression injects a deterministic timed-out process
runner only for the closed Cargo format profile. Project validation and docs
check complete normally; Cargo records `TimedOut`; the run fails; and the
durable schedule remains the exact three-step prefix.

The persisted report uses contract version `v4`, cites all three accepted
local-check results, and contains neither private process output nor fixture
paths.

## 7. Privacy And Redaction

No new raw payload field is stored or rendered. Errors remain stable and
payload-free. The timeout test explicitly proves that private stdout, private
stderr, and fixture paths do not enter serialized report content. Reports
continue to cite stable durable references rather than copying process output
or creating `EvidenceReference` values implicitly.

## 8. Test Coverage

The focused foreground suite now contains 43 passing tests, including:

- pre-result integrity failure on the first, second, and third steps;
- duplicate dispatch commitment rejection;
- composed Cargo timeout truth and v4 artifact integrity;
- dispatch, result, invocation, and successor substitution rejection;
- exact replay and create-only artifact reconciliation; and
- one-step and two-step compatibility coverage.

## 9. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1792685043041104000-2`
- approval: `approval/run-1792685043041104000-2/fix-approved`
- presentation: `presentation/fe27a69094e49333`
- presentation hash:
  `fe27a69094e4933390095e93844336177112a0c042c0fb4fad9d174282bef950`
- approval outcome: granted under delegated maintainer authority
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations

## 10. Validation

The following commands passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- focused `workflow-local-host` foreground admission tests: 43 passed
- `cargo test --workspace`
- `npm run check`
- `npm run check:integrations`
- `npm run check:docs`
- `git diff --check`

Opt-in live provider and sandbox tests remain behind their existing explicit
gates.

## 11. Remaining Limitations

This remains an exact private three-step trusted-host proof. It does not
provide an operating-system sandbox, independently attest the host toolchain,
authorize network access or provider mutation, expose released CLI behavior,
or generalize to arbitrary workflow topology.

## 12. Recommended Next Phase

Perform one final focused maintainer/security review of this blocker fix. If
accepted, close the trusted-host Cargo format composition lane before choosing
another runtime-composition item from the authoritative delivery queue.
