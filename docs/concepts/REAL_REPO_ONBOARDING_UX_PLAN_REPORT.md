# Real-Repo Onboarding UX Plan Report

## 1. Executive Summary

Real-repo onboarding UX planning is documented.

External testing against a public repository confirmed that `workflow-os validate`, `workflow-os init-repo-governance`, `workflow-os first-run`, approval-gated local execution, and durable state inspection already form a credible first-run product loop.

The same test identified P0 adoption gaps: existing `AGENTS.md` handling is safe but too sharp, `first-run` is useful but too generic, and the mock approval/audit workflow needs clearer separation from real first-run posture analysis.

## 2. Scope Completed

- Added [Real-Repo Onboarding UX Plan](../implementation-plans/real-repo-onboarding-ux-plan.md).
- Planned default preservation of existing repository agent guidance.
- Planned safe metadata-aware `first-run` detection and concrete review-only recommendations.
- Planned clearer output separation between real first-run posture and optional mock workflow demonstration.
- Updated roadmap and existing onboarding planning links.

## 3. Scope Explicitly Not Completed

- No implementation.
- No source-content inspection.
- No command execution.
- No local check execution.
- No provider calls.
- No automatic workflow generation.
- No automatic workflow registration or promotion.
- No workflow schema changes.
- No examples.
- No hosted or distributed runtime behavior.
- No writes.
- No release posture changes.

## 4. User Feedback Summary

The evaluated repository already had useful `AGENTS.md` content. Workflow OS correctly failed closed instead of overwriting it silently, but the practical next step was `--force`, which replaced existing project-specific guidance. The plan recommends preserving unmanaged content by default and appending/updating only the Workflow OS managed block.

The evaluator also found `workflow-os first-run` to be the strongest product signal. It gave useful governance posture without pretending to inspect source, run checks, or create state. The plan recommends making this output more concrete by inspecting only safe repository metadata such as package manifests, conventional source/test directory names, and CI workflow presence.

## 5. Recommended Implementation Order

1. Preserve existing `AGENTS.md` by default in `init-repo-governance` and `init-agent-harness`.
2. Add focused scaffold preservation tests.
3. Add safe metadata detection for `package.json` and TypeScript/npm recommendations.
4. Add bounded follow-up detection for Rust, Python, Go, and GitHub Actions.
5. Improve default human-facing first-run summary while preserving bounded detail and JSON.

## 6. Governed Dogfood Summary

- Workflow: `dg/d`.
- Run: `run-1783312329888246000-2`.
- Approval: `approval/run-1783312329888246000-2/planning-approved`.
- Approval outcome: granted under delegated maintainer authority after the complete approval handoff block was surfaced.

## 7. Commands Run And Results

- `npm run dogfood:benchmark -- phase-start --phase planning --state-dir /private/tmp/workflow-os-real-repo-onboarding-feedback-state --no-build ...`: passed.
- `./target/debug/workflow-os --project-dir ./dogfood/workflow-os-self-governance --state-dir /private/tmp/workflow-os-real-repo-onboarding-feedback-state --mock-all-local-skills approve run-1783312329888246000-2 approval/run-1783312329888246000-2/planning-approved --actor user/dogfood-reviewer --reason approved-real-repo-onboarding-feedback-planning`: passed.
- `npm run check:docs`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783312329888246000-2 --phase planning --state-dir /private/tmp/workflow-os-real-repo-onboarding-feedback-state --no-build`: passed.

## 8. Remaining Known Limitations

- Existing `AGENTS.md` preservation is planned but not implemented.
- Safe repo metadata detection is planned but not implemented.
- First-run output remains dense until a later implementation slice.
- Mock first-run workflow wording still needs implementation work to avoid product confusion.
- Workflow recommendations remain review-only and generic until metadata-aware recommendations are implemented.

## 9. Recommended Next Phase

Recommended next phase: existing agent-instruction preservation implementation.

Implement the smallest code slice first: append or update the Workflow OS managed block while preserving existing `AGENTS.md` content by default, keep `--force` replacement explicit, add dry-run messaging, and add focused tests. Do not implement metadata inspection, command execution, workflow generation, schemas, examples, hosted behavior, writes, or release posture changes in that slice.
