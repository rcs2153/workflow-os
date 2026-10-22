# Trusted-Host Terminal WorkReport Artifact Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; private artifact path accepted.**

The correction closes the operational success-signaling defect identified in
the implementation review. Report generation and artifact persistence failure
now produce bounded nonzero host outcomes without rewriting the already
committed terminal workflow result.

## 2. Scope Verification

The fix stayed within the approved unpublished host boundary. It changed only
the host posture mapping, focused trusted-host entry-point tests, the fix
report, and roadmap status.

It added no released CLI behavior, profile, artifact model or store, arbitrary
command, scheduler, provider, external write, SideEffect execution, schema,
example, hosted expansion, OpenShell integration, nested harness, or release
posture change.

## 3. Original Blocker Restatement

`artifact_posture(...)` previously returned fixed strings for every posture.
`run()` wrapped those strings in `Ok(...)`, so `GenerationFailed` and
`PersistenceFailed` exited successfully. Host automation could therefore
mistake an absent required artifact for a successful trusted-host operation.

The corrupt-history tests also accepted that zero-exit posture, masking the
contract violation. Direct coverage for valid conflicting and unreadable
duplicates was absent.

## 4. Host Failure Contract Assessment

The mapping now returns `Result<&'static str, &'static str>`. Persisted and
exactly reconciled artifacts remain successful. Generation and persistence
failures return `Err` for both fresh terminal execution and terminal replay,
using the binary's existing bounded failure path.

The fixed failure vocabulary is:

- `operation.terminal_report_generation_failed`;
- `operation.terminal_report_artifact_persistence_failed`;
- `admission.terminal_replay_report_generation_failed`; and
- `admission.terminal_replay_report_artifact_persistence_failed`.

No report, artifact, run, event, path, hash, or underlying storage detail is
rendered.

## 5. Terminal Truth Assessment

The host exit correction does not alter workflow truth. Tests prove that a
terminal failed run remains durably failed even when report generation cannot
complete. Duplicate persistence failures leave the run snapshot and event
count unchanged. No artifact failure path reruns a process, appends an event,
or converts a completed/failed run into another status.

This is the correct separation: the workflow terminal result is authoritative,
and the host operation still fails when its required artifact obligation is
not satisfied.

## 6. Duplicate Reconciliation Assessment

The new tests exercise `SqliteStateBackend` through the actual unpublished host
entry point.

- A payload that remains a valid artifact but differs from the newly composed
  artifact reaches the exact-equality conflict branch and exits nonzero.
- An unreadable JSON payload reaches the unreadable-duplicate branch and exits
  nonzero.

Both tests compare the raw persisted row after replay and prove it was not
overwritten, repaired, deleted, or recreated. They also prove event history is
unchanged.

## 7. Corrupt-History Assessment

All contradictory result/run, cancellation, and event-order fixtures now
require process non-success and empty stdout. Depending on which boundary
detects corruption first, stderr contains either the fixed invalid-state code
or fixed report-generation failure posture. Neither path leaks tampered data.

No corrupt-history fixture permits successful process exit or replacement
artifact persistence.

## 8. Privacy And Error Assessment

The implementation preserves fixed bounded output. Artifact failures emit
empty stdout and exact fixed stderr. Underlying `WorkflowOsError` details are
not promoted through the host surface.

The fix stores or prints no report contents, identifiers, file paths, command
output, source/spec contents, parser/provider payloads, environment values,
credentials, authorization headers, private keys, tokens, or approval reasons.

## 9. Test Quality Assessment

The focused suite now has 21 passing entry-point tests. It directly covers the
two previously missing duplicate branches and tightens every known
artifact-generation corruption path to require non-success.

The full workspace suite passes, including existing report, artifact,
executor, state, provider, hosted, and trusted-host tests. Opt-in live-provider
tests remain ignored by design and are unrelated to this private no-provider
path.

One test mutates a bounded known-limitation summary to create a different valid
artifact at the same relational key. Deserialization validation occurs during
the production duplicate read, so the fixture proves a valid conflict rather
than merely malformed storage.

## 10. Documentation Assessment

The original implementation review retains its historical `Needs blocker
fixes` verdict. The fix report records the correction and this review provides
independent fix-forward acceptance. The roadmap remains explicit that the
artifact path is private, local, closed-profile, and no-write.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- A future fault-injection boundary may test lower-level SQLite failures that
  occur before or after commit without corrupting rows manually.
- The private artifact still has no released inspect, rendering, or export
  surface.
- The report composer currently rehydrates once for composition and once for
  gate input; supported terminal history is immutable, but future broadening
  should avoid creating a mutable gap between those reads.

## 13. Governed Review Record

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1792668006342232000-2`
- Approval ID:
  `approval/run-1792668006342232000-2/review-scope-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/0e06982422705058`
- Review status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.
- Validation summary: formatting, strict clippy, the 21-test focused host
  suite, the complete workspace suite, documentation checks, and diff checks
  passed.

The agent inspected code, tests, and documentation and wrote this review
outside the kernel under the approved scope. The kernel coordinated the
proof-backed review gate and durable phase record; it did not inspect code,
execute checks, write files, commit changes, or perform provider activity.

## 14. Recommended Next Phase

Merge this accepted private artifact vertical slice. The dedicated draft-pull-
request live sandbox proof remains dependency-tracked and may run unchanged
when its least-privilege sandbox credential is available. Before another
provider mutation family or released artifact surface, refresh the active
delivery queue around the smallest remaining runtime-composition proof rather
than adding another model-only primitive family.
