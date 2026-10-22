# Trusted-Host Cargo Format Workflow Composition Final Blocker Fix Review

## 1. Executive Verdict

**Phase accepted; close the trusted-host Cargo format workflow composition
lane.**

The final narrow fix closes the remaining correctness blocker and both missing
proof gaps. Pre-result terminal failure is now validated against the last
durably scheduled step, direct duplicate dispatch commitments fail closed, and
the composed Cargo timeout path records terminal truth and persists one honest
v4 artifact. No blocker remains in the approved private three-step scope.

## 2. Scope Verification

The fix stayed within the approved private, local, exact three-step, no-write
boundary. It did not add another profile, arbitrary commands, caller-selected
topology, provider access, external writes, released CLI behavior, schemas,
examples, hosted execution, nested harness execution, or release changes.

The only production change is in the existing three-step history validator.
The remaining changes are focused tests and phase records.

## 3. Scheduled-Prefix Correctness

The validator now computes the durable `StepScheduled` count once and uses it
for both the in-progress invocation gap and terminal pre-result failure paths.
The corrected condition accepts a result-free integrity failure only on the
last step actually scheduled for the run, rather than only on the third
declared workflow step.

The surrounding invariants remain closed. The schedule must be an exact prefix
of project validation, docs check, and Cargo format. The attempted step must
have one deterministic invocation request, one causally ordered matching
dispatch, one matching failed invocation, and a terminal `RunFailed` event.
No local-check result is fabricated.

Focused tests prove the valid first-, second-, and third-step pre-result shapes.
The first- and second-step cases preserve only prior accepted results, retain
the exact scheduled prefix, produce bounded incomplete-work disclosure, and
cite no nonexistent result.

## 4. Dispatch Integrity

The new duplicate-dispatch corruption regression copies the first durable
invocation commitment onto the second dispatch in an otherwise completed run.
Terminal replay rejects the history before artifact recovery. This directly
proves that dispatch accounting cannot be satisfied by reusing a commitment
from another attempted step.

Existing dispatch executor-substitution, result/profile substitution,
invocation substitution, and successor-boundary corruption regressions remain
green. Together they cover the exact causal and identity claims made by the
private composition path.

## 5. Timeout And Artifact Integrity

The composed timeout regression uses the closed project-validation and docs
profiles normally, then injects a deterministic timed-out process runner for
the closed Cargo profile. The run records three scheduled steps and three
accepted results, with the Cargo result exactly `TimedOut`, then terminates as
`Failed`.

The persisted report uses contract version `v4`, cites all three accepted
local-check results, and contains neither private stdout, private stderr, nor
fixture paths. The test proves the complete composed boundary rather than
relying on a lower-level timeout unit test.

## 6. Privacy And Error Handling

No new caller payload or raw process field is introduced. Validation errors
remain stable and payload-free. The final regressions exercise rejection and
serialization surfaces without exposing repository paths, command output, or
mutated event payloads.

The report continues to cite durable references rather than copying process
output or creating `EvidenceReference` values implicitly.

## 7. Compatibility Assessment

The one-step and two-step trusted-host paths remain unchanged. Their existing
compatibility suites pass as part of the full workspace run. The three-step
registry remains an exact sibling rather than a generalized topology or
profile-extension mechanism.

## 8. Test Quality Assessment

The focused foreground-admission suite now passes 43 tests and includes direct
coverage for:

- pre-result integrity failure on every step;
- duplicate and substituted dispatch commitments;
- result, profile, invocation, and successor-boundary substitution;
- composed Cargo failure and timeout;
- restart-safe progression and terminal replay;
- create-only v4 artifact reconciliation and conflict refusal; and
- bounded report and process-output privacy.

The previously missing tests are present and exercise the production
composition boundary. No shallow-test blocker remains.

## 9. Validation Assessment

The reviewed revision passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- focused `workflow-local-host` foreground admission tests: 43 passed
- `cargo test --workspace`
- `npm run check`
- `npm run check:integrations`
- `npm run check:docs`
- `git diff --check`

Opt-in live provider and sandbox tests remain behind their existing explicit
gates and are not required for this local no-provider phase.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Keep this as an exact sibling until evidence supports a separately planned
  topology abstraction.
- Preserve the distinction between trusted-host final-state detection and
  operating-system containment.
- Continue treating host toolchain identity as self-committed, not
  independently attested.

## 12. Governed Review Record

- workflow: `dg/review`
- run: `run-1792685842149689000-2`
- approval:
  `approval/run-1792685842149689000-2/review-scope-approved`
- presentation: `presentation/a5020bd62c458f8d`
- presentation hash:
  `a5020bd62c458f8dd070e9283c2b9fc6ae99cf5a85bef37ed498db5850df26d0`
- approval outcome: granted under delegated maintainer authority for the exact
  persisted review-only scope
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations
- out-of-kernel work: source, test, history, report, and validation evidence
  were inspected by the delegated maintainer; the kernel governed scope and
  approval but did not perform the review actions

## 13. Recommended Next Phase

Close this composition lane. The first active delivery-queue item remains the
dedicated draft-pull-request live sandbox proof when its least-privilege
credential dependency is available. If that dependency remains unavailable,
perform explicit queue maintenance before selecting the next unrelated
no-write runtime phase; do not infer a new phase from historical lane text.
