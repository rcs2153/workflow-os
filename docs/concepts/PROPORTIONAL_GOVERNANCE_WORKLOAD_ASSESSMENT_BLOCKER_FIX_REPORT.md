# Proportional Governance Workload Assessment Blocker Fix Report

## 1. Executive Summary

The two blockers from the proportional-governance workload-assessment review
are fixed. Fingerprint field framing is architecture-stable, and inferred
action posture now has truthful reason provenance distinct from explicit
workflow declarations.

The boundary remains model-only, in-memory, assessed-not-enforced, and not
persisted.

## 2. Blockers Fixed

### Architecture-Stable Fingerprint Framing

The original helper hashed `usize::to_be_bytes()` field lengths. Because
`usize` width follows host architecture, the same validated input could produce
different fingerprints on 32-bit and 64-bit hosts.

The v1 format now frames every label and value with a fixed `u64` big-endian
length. A known-vector regression pins the complete bounded-read baseline to:

```text
77748614823b70948a582a314fe9059152eaf21f948d28357bae8a930d28d25c
```

### Truthful Assessment Reason Provenance

The original helper merged inferred action posture into the selector's
workflow-declared requirement. An external mutation could therefore emit
`WorkflowRequirement` without a workflow declaration.

`ProportionalGovernanceDecisionInput` now includes a distinct
`workload_assessment` requirement and
`GovernanceDecisionReason::WorkloadAssessment`. The workload helper sends
action-derived posture through that source while explicit workflow minima
remain in `workflow`.

## 3. Compatibility

The selector input is preview Rust API and had no runtime callers. Its focused
tests and the workload-assessment helper are the only constructors in the
repository. Existing execution, projection, provider, persistence, schema, and
CLI behavior remain unchanged.

Decision reason serialization gains the additive `workload_assessment` value.
No persisted or public schema consumes it in this phase.

## 4. Tests Added

Focused regressions prove:

- the complete v1 fingerprint matches a fixed known vector;
- external mutation emits `WorkloadAssessment`;
- inferred external mutation does not emit `WorkflowRequirement`;
- an explicit workflow minimum still emits `WorkflowRequirement`;
- every existing selector reason remains representable;
- existing deterministic, monotonicity, invalidation, and privacy tests pass.

## 5. Scope Explicitly Not Added

The fix does not add runtime enforcement, onboarding integration, repository
scanning, persistence, events, schemas, CLI/UI behavior, automatic approvals,
provider calls, new mutation families, hosted administration, or release
changes.

## 6. Validation

Validation passed:

- `cargo test -p workflow-core --test proportional_governance_assessment --test proportional_governance`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

## 7. Governed Phase Evidence

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1783929374231221000-2`.
- Approval ID: `approval/run-1783929374231221000-2/fix-approved`.
- Approval presentation: `presentation/21787315a02b05d4`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Out-of-kernel work: Codex changed source, tests, and documentation and ran
  validation. The kernel coordinated governance only.
- Report posture: no runtime WorkReport artifact was generated or persisted.

## 8. Recommended Next Phase

Perform a focused maintainer re-review. Do not begin onboarding or runtime
integration until both fixes are independently accepted.
