# Trusted-Host Terminal WorkReport Artifact Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to the private trusted-host terminal WorkReport
artifact implementation.**

The plan defines the smallest credible persistence composition over accepted
runtime primitives. It corrects the private report's storage claim before the
first write, reuses the existing artifact model, integrity gates, and SQLite
store, and keeps artifact failure separate from terminal workflow truth.

## 2. Scope Verification

The plan remains private, local, and limited to the two existing closed
no-write trusted-host profiles. It does not authorize a released CLI surface,
another report or store model, arbitrary commands, scheduling, provider
access, credentials, external writes, SideEffect execution, workflow schema
changes, examples, hosted expansion, OpenShell, or nested harnesses.

The automatic write is bounded to the unpublished trusted-host operation after
its run has terminally committed. It does not enable automatic report
artifacts for ordinary executor paths.

## 3. Architecture Assessment

The proposed boundary correctly composes:

```text
terminal durable run
  -> exact history validation
  -> deterministic private v2 WorkReport
  -> validated WorkReportArtifactRecord
  -> existing artifact integrity gates
  -> existing SQLite WorkReportArtifactStore
```

This is preferable to creating another artifact abstraction. The current
`SqliteStateBackend` already implements the required stores, and the existing
authoritative artifact path supplies a reviewed create-only duplicate
reconciliation pattern.

## 4. Report Contract Correction

The v2 correction is required and sufficient. Persisting the current v1 report
would preserve a false statement that the report remains in memory and is not
written as an artifact. The plan correctly requires a new domain separator,
contract version, deterministic report identity, and storage-neutral
limitation before any artifact write.

This is not a public compatibility break. The private v1 report has no released
artifact or inspection contract, and no migration is necessary.

## 5. Gate And Policy Assessment

The implementation must invoke
`write_report_artifact_with_explicit_integrations(...)` with no provider
integration and must not infer that an empty current SideEffect set permits a
gate bypass. Citation, approval-linkage, and decision requirements remain
enabled so later report evolution fails closed.

The plan now names the exact high-assurance posture:
`WorkReportArtifactHighAssuranceDisclosurePolicy::disabled()`. That is the
appropriate fixed policy for these closed no-write profiles and removes an
otherwise avoidable implementation ambiguity.

## 6. Terminal And Failure Semantics

The host already proves that passed checks complete, concrete failed checks
terminalize as failed runs, and both paths reach report composition. Internal
execution errors without a concrete result remain errors and do not fabricate
a terminal report.

The proposed artifact helper runs only after terminal commit. Report, gate,
store, or reconciliation failure cannot rewrite run status, append a workflow
event, mutate the snapshot, or rerun the process. A bounded host failure posture
may fail the unpublished operation while preserving the authoritative terminal
workflow result.

## 7. Replay, Concurrency, And Recovery

Create-only storage plus read-after-duplicate full equality is the correct
local persistence posture. It handles:

- crash after terminal commit by composing and writing on replay;
- crash after artifact commit by returning `AlreadyPersisted`;
- equal concurrent writers by converging on one artifact; and
- conflicting, unreadable, or invalid duplicates by failing closed.

The implementation must compare the complete validated artifact, not selected
identity fields, and must never overwrite or repair a conflicting record.

## 8. Corrupt-History Requirement

The prior composition review's non-blocking follow-up becomes a required
pre-write proof. Full entry-point fixtures must show that contradictory result,
invocation, run, cancellation, and event-order histories fail before the
artifact store is called. This closes the trust-boundary expansion identified
by the earlier review.

## 9. Privacy And Output Assessment

The planned artifact remains reference-oriented and payload-free. It must not
store or print command output, local-check summaries, source or spec contents,
paths, environment values, credentials, approval reasons, policy text, or
caller-authored unbounded prose.

Fixed host output may disclose only persistence posture. Debug and errors must
redact report contents, artifact contents, and identifiers. The phase makes no
encryption-at-rest, multi-user authorization, retention, DLP, or regulated-data
claim.

## 10. Test Assessment

The proposed test matrix is proportionate to the new persistence boundary. It
covers passing, failed, and timed-out terminal truth; non-terminal exclusion;
v2 report honesty; gate invocation; exact replay; concurrency; duplicate
conflict; both crash windows; full corrupt-history rejection; fixed output;
privacy; event immutability; and full regression validation.

Implementation should include at least one store spy or fault seam proving that
generation and gate failures occur before the write call. Existing SQLite
fault/reconciliation seams should be reused where sufficient rather than
adding a production abstraction solely for tests.

## 11. Planning Blockers

None.

## 12. Non-Blocking Follow-Ups

- Decide during implementation whether the private result preserves an
  underlying stable store code or maps it into one artifact namespace; either
  choice must remain non-leaking.
- Keep released report inspection/export deferred until a separate user-surface
  plan addresses authorization, retention, and sensitivity.
- Do not generalize this helper to ordinary executor paths without a separate
  review of contract selection and artifact obligations.

## 13. Governed Review Record

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1792666387726892000-2`
- Approval ID:
  `approval/run-1792666387726892000-2/review-scope-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/4188e4d44c8d0983`
- Review status: `Completed`
- Validation summary: documentation and diff validation passed.

The agent inspected code, tests, plans, and prior review evidence and wrote this
review outside the kernel under the approved scope. The kernel coordinated the
proof-backed review gate and durable phase record; it did not inspect code,
write files, execute validation, commit changes, or perform provider activity.

## 14. Recommended Next Phase

Implement the private trusted-host terminal WorkReport artifact vertical slice
exactly as reviewed. Begin with the v2 report correction, then add the bounded
artifact result, existing-gate composition, exact duplicate reconciliation,
host integration, and required corruption/recovery tests.

Do not add released CLI rendering or export, another profile, arbitrary
commands, scheduling, providers, external writes, schemas, examples, hosted
expansion, OpenShell, or nested harness execution.
