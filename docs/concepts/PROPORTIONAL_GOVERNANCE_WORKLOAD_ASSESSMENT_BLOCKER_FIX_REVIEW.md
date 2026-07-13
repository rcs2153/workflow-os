# Proportional Governance Workload Assessment Blocker Fix Review

## 1. Executive Verdict

**Blockers fixed; proceed to deterministic read-only onboarding fact derivation.**

Both blocker corrections are narrow, deterministic, and independently covered
by regressions. The workload assessment remains model-only, in-memory,
assessed-not-enforced, and not persisted.

## 2. Scope Verification

The fix stayed inside the approved blocker scope. It changed fixed-width
fingerprint framing, selector reason provenance, focused tests, and phase
documentation.

It did not add runtime enforcement, onboarding integration, repository
scanning, persistence, events, schemas, CLI/UI behavior, automatic approvals,
provider calls, mutation-family expansion, hosted administration, or release
changes.

## 3. Fingerprint Blocker Assessment

The v1 fingerprint no longer depends on host pointer width. Every label and
value is framed with a fixed `u64` big-endian byte length before hashing. Label
and value framing remains unambiguous, and the versioned algorithm domain is
still included.

The bounded-read baseline is pinned to a complete known vector:

```text
77748614823b70948a582a314fe9059152eaf21f948d28357bae8a930d28d25c
```

Existing invalidation tests continue to prove that the immutable definition
root and each modeled decision-relevant fact or minimum changes the
fingerprint. The architecture-stability blocker is fixed.

## 4. Reason Provenance Blocker Assessment

`ProportionalGovernanceDecisionInput` now has a distinct
`workload_assessment` requirement, mapped to the stable
`WorkloadAssessment` reason. Action-derived posture uses that input. Explicit
workflow minima continue to use the separate `workflow` input and
`WorkflowRequirement` reason.

Focused regressions prove both directions:

- inferred external mutation emits `WorkloadAssessment` and does not emit
  `WorkflowRequirement`;
- an explicit workflow minimum emits `WorkflowRequirement` and does not emit
  `WorkloadAssessment` when inference itself is quiet.

The false provenance blocker is fixed.

## 5. Compatibility And Safety Assessment

The additive selector input and reason affect preview Rust vocabulary only.
Repository search found no runtime constructor outside the selector tests and
the workload-assessment helper. No persisted format or public workflow schema
consumes the new reason in this phase.

The accepted selector still composes profile, workload assessment, workflow,
policy, authority, evidence/check, sensitivity, SideEffect, runtime,
prior-decision, and steward requirements monotonically. Inference cannot weaken
an explicit minimum or substitute approval for failed checks or unavailable
authority.

## 6. Privacy And Product Boundary Assessment

The fixes add no payload-bearing fields. The assessment still accepts bounded
typed facts and an immutable definition root, redacts hashes from `Debug`, and
stores no prompts, source contents, provider payloads, command output, paths,
environment values, credentials, or tokens.

Execution disposition remains independent from disclosure presentation.
`Proceed + Visible` is a valid result; UI visibility is not execution authority.
The model does not infer from natural language and cannot approve work.

## 7. Test Quality Assessment

The blocker regressions are direct and meaningful:

- fixed known v1 fingerprint vector;
- inferred workload-assessment reason present;
- false workflow reason absent;
- explicit workflow reason preserved;
- every selector reason remains representable;
- full existing monotonicity, invalidation, privacy, executor, provider-write,
  evidence, approval, and workspace suites remain green.

Explicit unsupported action and SideEffect variant tests remain useful but
non-blocking because the implementation maps both to denial and the existing
matrix covers unsupported selector failure behavior.

## 8. Documentation Assessment

The implementation report preserves the original blocker history and records
the actual full validation results. The original review remains unchanged as a
historical blocker verdict and links forward to this acceptance review.

Roadmap and plan status remain honest: deterministic typed assessment exists,
but safe repository-metadata derivation, onboarding integration, runtime
enforcement, persistence, UI, and schemas do not.

## 9. Remaining Non-Blocking Follow-Ups

- Add explicit unsupported workload action and SideEffect mapping tests.
- Decide whether assessed/not-enforced and not-persisted posture must become
  serialized fields before machine-facing projection.
- Keep deterministic metadata derivation separate from model or
  natural-language inference.
- Reassessment may recommend or escalate but must never weaken explicit
  profile, workflow, policy, authority, evidence/check, sensitivity,
  SideEffect, prior-decision, or steward minima.

## 10. Recommended Next Phase

Implement or plan the smallest deterministic read-only onboarding fact
derivation boundary. It should derive bounded workload facts from validated,
safe repository and workflow metadata, bind them to the immutable definition
root, disclose unknowns, and call the accepted assessment helper.

Do not add runtime enforcement, automatic approvals, arbitrary source reading,
model inference, provider writes, new mutation families, schemas, or UI in that
phase.

## 11. Validation

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed before this review and rerun at review close;
- `git diff --check`: passed before this review and rerun at review close.

## 12. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783931187606627000-2`.
- Approval ID:
  `approval/run-1783931187606627000-2/review-scope-approved`.
- Approval presentation: `presentation/5dfdabddc9250f7e`.
- Approval outcome: granted with persisted presentation proof under delegated
  maintainer authority.
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.
- Out-of-kernel work: Codex inspected source, tests, documentation, and
  validation results and authored this review. The kernel coordinated
  governance only.
- Report posture: no runtime WorkReport artifact was generated or persisted.
