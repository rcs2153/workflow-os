# Proportional Governance Core Model Report

## 1. Executive Summary

Workflow OS now has a model-only proportional-governance decision boundary. A
pure helper selects the strictest applicable interaction mode from typed,
bounded governance requirements and preserves any prior stricter decision.

This phase does not activate quiet success or change runtime behavior.

## 2. Scope Completed

- Added ordered governance interaction modes: quiet capture, visible
  disclosure, blocking approval, and denial.
- Added corresponding ordered risk classes.
- Added typed posture requirements for profile, workflow, policy, authority,
  evidence/checks, sensitivity, SideEffect posture, and runtime escalation.
- Added stable payload-free decision reason codes.
- Added a pure strictest-posture selector.
- Added prior-decision monotonicity so reassessment cannot silently downgrade an
  active decision.
- Added focused model, failure, serde, and non-leakage tests.

## 3. Scope Explicitly Not Completed

- No executor or runtime integration.
- No quiet-success CLI or rendering behavior.
- No automatic or model-generated approvals.
- No change to approval defaults.
- No workflow schema fields or runtime configuration.
- No persistence, workflow events, audit projection, or metrics collection.
- No provider writes or mutation-family expansion.
- No hosted stewardship or enterprise administration.
- No examples or release posture changes.

## 4. Model Summary

`ProportionalGovernanceDecisionInput` accepts only enums and an optional prior
interaction mode. It stores no prompts, paths, source contents, command output,
provider payloads, credentials, or caller-supplied summaries.

`select_proportional_governance` derives the profile minimum, evaluates each
typed requirement independently, selects the maximum ordered mode, and then
applies the prior decision as a floor. `Unsupported` requirements return the
stable error code `governance.proportional.requirement.unsupported`.

## 5. Determinism And Escalation

The selector is pure and has no hidden state. Identical typed inputs produce the
same result. Requirement fields cannot weaken one another because selection is
an ordered maximum. A prior disclosure, approval, or denial remains a minimum
for reassessment, providing monotonic escalation without implementing runtime
checkpoints.

## 6. Privacy And Redaction

Inputs, outputs, reason codes, Debug output, and serialized forms contain only
bounded enum vocabulary. Errors do not include the requirement source or any
caller payload. The model cannot store raw evidence, logs, source content,
provider data, approval reasons, or secrets.

## 7. Test Coverage

Focused tests cover quiet capture, strictest-requirement selection, profile
minimums, runtime escalation, prior denial, explicit denial, unsupported
requirements, serde round trips, invalid serialized decisions, bounded
serialization, and every reason source.
Existing workspace tests provide regression coverage for governance profiles,
approvals, evidence, reports, SideEffects, adapters, and runtime behavior.

## 8. Commands And Results

- `cargo test -p workflow-core --test proportional_governance`: passed, 14 tests
  after blocker fixes.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live tests remained ignored by their
  existing environment gates.
- `npm run check:docs`: passed.

## 9. Known Limitations

- Existing governance profiles remain disclosure vocabulary and are not
  automatically enforced by an executor.
- Requirement derivation from policy, authority, evidence, checks, sensitivity,
  and SideEffect models is deferred.
- No durable decision or escalation event exists.
- Quiet capture is not active on any runtime path.
- No operator output or friction/completeness metric is produced.

## 10. Recommended Next Phase

Perform a focused maintainer review of the proportional-governance core model.
Review deterministic ordering, fail-closed handling, profile compatibility,
monotonic escalation, serde safety, and whether the typed requirement boundary
is sufficiently explicit before any read-only projection or runtime integration.

## 11. Governed Phase Evidence

- Workflow ID: `dg/implement`.
- Run ID: `run-1783805208237079000-2`.
- Approval ID:
  `approval/run-1783805208237079000-2/implementation-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/809c051fcdd8167c`.
- Out-of-kernel work: Codex performed repository edits and validation commands;
  the kernel governed scope and approval but did not edit files or run checks.
- Event and terminal summary: completed with 39 ordered events, one approval,
  zero retries, and zero escalations. Approval presentation was proof-enforced
  and the approval event marker was present.
