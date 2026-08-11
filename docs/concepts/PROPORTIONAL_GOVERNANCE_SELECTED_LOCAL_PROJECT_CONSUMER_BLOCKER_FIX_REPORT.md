# Proportional-Governance Selected Local Project Consumer Blocker Fix Report

## 1. Executive Summary

The selected local project-validation consumer no longer accepts a
caller-authored governance evaluation time. Core now selects a fresh timestamp
inside the initial route call and independently inside every approval decision
call. The generic registered-source APIs remain unchanged.

## 2. Blocker Fixed

The original composition exposed `evaluated_at` on
`LocalSelectedProjectValidationGovernanceRequest` and
`LocalSelectedProjectValidationArtifactDecisionInput`. Because the fixed source
used that value as its observation time, callers could choose the freshness
clock for a path documented as Core-owned.

Both public fields are removed. A selected-consumer caller can no longer
backdate or future-date either the initial assessment or decision-time
reassessment.

## 3. Implementation Approach

- Initial routing calls `Timestamp::now_utc()` inside Core before constructing
  the Core-owned route options.
- Each granted decision calls `Timestamp::now_utc()` after persisted approval
  presentation proof succeeds and before immutable-bundle reassessment and the
  canonical project-validation check.
- Each denied decision obtains its internal decision timestamp only after proof
  succeeds and preserves the accepted source-free, check-free denial path.
- Existing explicit generic APIs continue to accept evaluation time because
  their caller-visible source boundary was not part of this fix.

## 4. Validation Boundary

The selected public request types now contain only execution, approval, report,
and selected artifact-gate inputs already required by the closed composition.
Source identity, registration, facts, observation time, and evaluation time are
owned by Core. Presentation proof still precedes decision-time project access,
check execution, source observation, and approval mutation.

## 5. Privacy And Compatibility

No raw facts, paths, command output, report text, environment values, provider
payloads, credentials, or timestamps were added to Debug or errors. Existing
selected-consumer behavior, generic registered-source APIs, executor APIs, and
CLI behavior remain unchanged.

## 6. Test Coverage

The five focused selected-consumer tests compile against the fact-free public
request shape and continue to cover complete two-gate success, denial without
recheck or writes, missing presentation proof, relevant-definition
invalidation, and failed decision-time validation before mutation.

Removing the public fields is structural proof that selected callers cannot
supply a stale, backdated, or future-dated evaluation time. The existing generic
API tests retain deterministic caller-supplied timestamps and remain the
lower-level compatibility boundary.

## 7. Commands And Results

- `cargo fmt --all --check`: passed.
- Focused selected-consumer local-executor tests: 5 passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed with the
  repository toolchain and an isolated target directory.
- `cargo test --workspace`: passed with the repository toolchain and an
  isolated target directory; opt-in live tests remained ignored as designed.
- `npm run check:docs`: passed under the repository Node 20 toolchain.
- `git diff --check`: passed.

## 8. Governed Phase Record

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1786443809809049000-2`.
- Approval ID: `approval/run-1786443809809049000-2/fix-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/bf2c6e68b0fb8f45`.
- Terminal status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, no retries, and no
  escalations.
- Approval-presentation enforcement: proof enforced with the presentation
  marker present in the durable event trail.

Repository edits and validation commands are executed by the delegated
maintainer outside the kernel. The kernel governs scope and approval and keeps
the durable phase trail; it does not edit files, execute checks, mutate git
state, push the branch, or update the pull request.

## 9. Remaining Limitations

- CLI adoption remains unimplemented.
- The selected consumer remains explicit, local, one-step, and opt-in.
- Runtime-fact snapshots remain call-local evidence metadata, not reusable
  authority.
- No provider, OpenShell, SideEffect execution, new mutation family, schema,
  example, or hosted behavior is introduced.

## 10. Recommended Next Phase

Perform a focused blocker-fix review. If accepted, merge the selected-consumer
composition and plan CLI adoption as a separate compatibility-sensitive phase.
