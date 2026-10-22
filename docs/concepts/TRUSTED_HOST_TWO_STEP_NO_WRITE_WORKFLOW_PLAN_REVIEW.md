# Trusted-Host Two-Step No-Write Workflow Plan Review

## 1. Executive Verdict

**Plan accepted with one incorporated compatibility correction; proceed to the
bounded two-step no-write trusted-host implementation.**

The plan selects the smallest runtime-composition proof that materially
advances the trusted-host boundary. It reuses the two accepted closed profiles,
keeps Core authoritative for progression, and requires atomic, restart-safe
handoff between steps before final report and artifact production.

## 2. Scope Verification

The plan remains unpublished, local, closed-profile, and no-write. It does not
authorize another profile, arbitrary command, caller-authored step ordering,
parallelism, branching, provider access, credentials, SideEffect execution,
external writes, automatic approval, released CLI behavior, schema changes,
examples, hosted expansion, nested harnesses, or release changes.

The bounded process loop is limited to at most two operations for one explicit
run. It is not queue polling, a daemon, a generic scheduler, or automatic run
discovery.

## 3. Runtime Composition Assessment

The selected order is coherent:

```text
local/workflow-os-validate@v0
  -> local/check-docs@v0
  -> terminal WorkReport
  -> private SQLite artifact
```

Both profiles already have reviewed command, timeout, root, environment,
network, current-authority, required-context, result, and redaction contracts.
Composing them sequentially proves a new runtime property without adding a new
execution capability.

## 4. Authority Boundary Assessment

The new mode correctly removes caller-provided step and profile selection.
The caller supplies only run selection, immutable bundle identity, trusted
timestamps, actor/correlation identity, roots, and the two explicit executable
paths. Core rehydrates the immutable workflow, derives the only runnable step,
and selects its exact profile from a closed registry.

This is essential. Retaining a caller-authored `step-id` would let the host
claim progression rather than prove it from durable governed state.

## 5. Sequential Transition Assessment

The required first-step success transaction is sound:

1. record the payload-free concrete check result;
2. record the matching invocation success; and
3. schedule the exact second immutable step.

It must not complete the run. Failure or timeout instead commits the result,
failed invocation, and failed run without scheduling step two. Step-two success
commits the result, invocation success, and run completion.

Implementation should reuse the generic executor's accepted progression rules
or a shared internal helper. Copying a separate approximation would create
semantic drift and must stop the phase for review.

## 6. Restart And Replay Assessment

Fresh re-admission after every committed step is the correct boundary. The
process retains no reusable continuation capability and does not infer step
two from memory. A crash after first-step commit therefore resolves step two
from SQLite; terminal replay executes neither process and reconciles only the
artifact.

The plan correctly requires rejection of skipped, reordered, duplicated,
substituted, or contradictory histories before additional execution.

## 7. Report Compatibility Correction

The initial plan draft proposed generalizing the private v2 report over an
ordered result set. That could have changed deterministic report identity for
an existing one-step terminal run and caused a false artifact conflict on
replay.

The plan now preserves the complete v2 one-step material and identity
unchanged and requires a distinct private v3 commitment for the two-step
topology. V3 binds the immutable workflow and bundle, ordered steps and
profiles, concrete result events, matching invocation terminals, and run
terminal. No migration or v2 rewrite is authorized.

This incorporated correction removes the planning blocker.

## 8. Artifact Assessment

The plan reuses the accepted artifact record, integrity gates, create-only
SQLite store, exact duplicate reconciliation, and nonzero host failure
contract. Artifact generation remains post-terminal and cannot alter run
truth, append workflow events, rerun a process, overwrite a conflict, or
fabricate a result for an unattempted second step.

Where practical, implementation should derive report and gate input from one
validated terminal-history object. That resolves the accepted non-blocking
double-read follow-up without adding a public abstraction.

## 9. Privacy And Error Assessment

The allowed persisted material remains reference-oriented and payload-free.
The plan excludes command output, source/spec contents, parser/provider
payloads, paths, environment values, credentials, approval reasons, policy
text, and arbitrary report prose.

Fixed error and host posture remain appropriate. Debug and serialization must
not expose registry internals, executable paths, result contents, or IDs not
already authorized by the private model.

## 10. Test Assessment

The test matrix covers the new trust boundaries:

- ordered two-step success;
- first- and second-step failure;
- atomic intermediate progression;
- restart after the first commit;
- terminal replay;
- duplicate and corrupt history;
- closed registry rejection;
- Core-owned step selection;
- v3 report identity and v2 replay compatibility;
- artifact failure isolation;
- one-step regression; and
- privacy and workspace regression.

Implementation must invoke the unpublished binary for the end-to-end proofs,
not only private helpers.

## 11. Planning Blockers

None after the v3 compatibility correction incorporated into the plan.

## 12. Non-Blocking Follow-Ups

- A future phase may decide whether the temporary omitted-profile alias for the
  one-step docs-check path should be removed.
- A reviewed fault-injection seam could strengthen crash-before-commit proof if
  existing SQLite transaction tests cannot demonstrate it cleanly.
- Public report inspection remains a separate authorization, sensitivity, and
  retention design problem.

## 13. Governed Review Record

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1792668925328160000-2`
- Approval ID:
  `approval/run-1792668925328160000-2/review-scope-approved`
- Approval outcome: granted under delegated-maintainer authority with
  persisted presentation proof `presentation/cba23a7ed189886a`
- Review status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.
- Validation summary: `npm run check:docs` and `git diff --check` passed.

The agent inspected the implementation plan, accepted runtime code, tests, and
prior review evidence outside the kernel. The kernel coordinated the bounded
review gate and durable event record; it did not inspect code, edit files, run
checks, commit changes, or perform provider activity.

## 14. Recommended Next Phase

Implement the bounded two-step no-write trusted-host vertical slice exactly as
reviewed. Start with the closed registry, Core-owned topology/current-step
derivation, and atomic intermediate-step transition. Then add the bounded
process loop, private v3 report composition, accepted artifact reuse, and full
entry-point tests.

Do not add arbitrary commands, another profile, providers, writes, public CLI
behavior, schemas, examples, hosted expansion, parallelism, or nested harness
execution.
