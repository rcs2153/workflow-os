# Trusted-Host Project-Validation Profile Plan Blocker Fix Review

## 1. Executive Verdict

**Accepted; proceed to one bounded implementation vertical slice.**

The corrected plan closes both blockers without widening the trusted-host
surface. It now defines truthful local-check terminalization at the correct
private adaptation boundary and an exact, testable selector compatibility
rule.

## 2. Scope Verification

The correction remains planning-only. It does not authorize Rust behavior,
arbitrary commands, new profile families, providers, network access, source
writes, report artifacts, schemas, released CLI behavior, scheduling, or
hosted expansion.

## 3. Non-Passing Result Assessment

The private Core-owned trusted-host adapter is the correct boundary.
`LocalCheckResult` must remain a structured result because authoritative
governance APIs legitimately consume failed and timed-out checks as data.
Trusted-host skill execution, however, must accept only `Passed` as attempt
success.

The corrected plan requires one handler invocation, no rerun, fixed
non-leaking failure, and no raw-output persistence. Applying the adapter to
both accepted profiles closes the existing docs-check truth gap as well as the
new project-validation path.

## 4. Selector Compatibility Assessment

The selector rule is exact:

- project validation requires explicit
  `--profile workflow-os-project-validation`;
- docs-check accepts explicit `--profile docs-check`; and
- omission is a temporary compatibility alias for docs-check only.

Mixed argument families, unknown selectors, and implicit project-validation
selection fail before backend opening or state creation. The rule does not
infer authority from ambient files or argument coincidence.

## 5. Authority And Lifecycle Assessment

The correction preserves the distinction between closed execution binding and
additional current-authority facts. The new profile still requires a separate
exhaustive explicit-zero Core decision and distinct source/contract identity.

The accepted atomic request/start/terminal lifecycle and exact replay posture
remain unchanged. Failed and timed-out checks now have an explicit path to
`SkillInvocationFailed` and `RunFailed` rather than completion.

## 6. Privacy Assessment

The plan requires stable failure codes and fixed messages. Paths, executable
hashes, stdout, stderr, summaries, environment values, and command transcripts
remain excluded from errors and durable failure events.

## 7. Test Assessment

The corrected tests cover both profiles, failed and timed-out truth, single
execution, failed-run replay, bounded failure data, explicit selection, the
temporary docs-check alias, mixed-argument rejection, and rejection before
state creation. This is sufficient for the bounded implementation.

## 8. Remaining Blockers

None.

## 9. Non-Blocking Follow-Ups

- Plan terminal WorkReport composition before broader trusted-host adoption.
- Remove the unpublished compatibility alias only through an explicit later
  cleanup.
- Keep final validation-to-process-entry race hardening separate.

## 10. Recommended Next Phase

Implement the project-validation profile and shared trusted-host result adapter
as one unpublished foreground-binary vertical slice. Do not add another
profile or generic command host.

## 11. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Rust checks are not required for this documentation-only re-review.

## 12. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792661948735126000-2`.
- Approval:
  `approval/run-1792661948735126000-2/review-scope-approved`.
- Approval presentation: `presentation/964418c01c7674a7`.
- Approval outcome: granted under delegated-maintainer authority with
  persisted presentation proof.
- Phase status: `Completed`.
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  proof-enforced approval presentation.
- Validation summary: documentation and diff checks passed; Rust checks were
  not required because this phase changed documentation only.
- Out-of-kernel work: code/plan/test inspection, review authoring, validation,
  git, and later PR operations are performed by the delegated maintainer while
  Workflow OS governs the review scope.
