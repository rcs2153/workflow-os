# Trusted-Host Terminal WorkReport Artifact Implementation Review

## 1. Executive Verdict

**Needs blocker fixes.**

The private artifact composition is narrow, deterministic, and structurally
sound, but the unpublished host currently returns process success when report
generation or artifact persistence fails. That contradicts the accepted plan's
failure contract and can let host automation mistake a failed artifact
obligation for a successful operation.

## 2. Scope Verification

The phase stayed within the accepted private local boundary. It did not add a
released CLI surface, another profile, arbitrary commands, scheduling, provider
access, credentials, external writes, SideEffect execution, schemas, examples,
hosted expansion, OpenShell, nested harnesses, or release changes.

The added `rusqlite` and `serde_json` dependencies are test-only and already
exist in the workspace lockfile. They support direct corruption and recovery
fixtures rather than production behavior.

## 3. Architecture Assessment

The implementation correctly reuses the existing private terminal report
composer, `WorkReportArtifactRecord`, explicit artifact integration gate,
`WorkReportArtifactStore`, and `SqliteStateBackend`. It adds no new report,
artifact, or storage primitive.

The private result exposes a closed posture, optional validated artifact, and
optional stable error. Its custom `Debug` implementation discloses only posture,
artifact presence, and stable error code.

## 4. Report Contract Assessment

The v2 correction is implemented correctly:

- the report-material domain separator is v2;
- the private contract version is v2;
- deterministic report identity changes with the v2 commitment; and
- the persisted report no longer contains the false statement that it remains
  only in memory.

The replacement limitation accurately states that no released inspection or
export surface exists.

## 5. Artifact Gate Assessment

The helper constructs and validates `WorkReportArtifactRecord`, reconfirms the
run identity, and invokes `write_report_artifact_with_explicit_integrations(...)`
with:

- provider integration disabled;
- all SideEffect citations required;
- approval references required for approval-gated SideEffects;
- decisions required for approved or denied SideEffects; and
- high-assurance disclosure policy explicitly disabled for the closed no-write
  profiles.

The helper does not bypass the generic gate merely because current reports have
no SideEffect citations.

## 6. Replay And Recovery Assessment

Create-only storage and read-after-duplicate full artifact equality are
implemented. Focused host tests prove:

- exact terminal replay returns `AlreadyPersisted` without rerunning the process
  or appending events;
- replay after terminal commit with a missing artifact writes exactly one
  artifact without changing event history; and
- concurrent equal recovery callers converge on one persisted artifact and one
  exact-replay result.

The implementation never overwrites a duplicate artifact.

## 7. Corrupt-History Assessment

Entry-point tests remove the original artifact, tamper durable history, and
reinvoke the host. They cover contradictory passed/failed result and run truth,
concrete result plus cancellation, and non-monotonic result/invocation order.
Each case leaves the artifact store empty.

The integrity checks themselves use stable non-leaking errors. A separate
result-shape unit test covers additional status and order combinations.

## 8. Blocker Finding

### B1. Artifact failure exits successfully

`artifact_posture(...)` maps `GenerationFailed` and `PersistenceFailed` to
fixed strings, but its return type is `&'static str`. `run()` wraps that result
in `Ok(...)` for both fresh terminal operations and terminal replay. The binary
therefore exits successfully even though the artifact obligation failed.

This violates the accepted plan, which requires a bounded non-success host
result for report generation or persistence failure while preserving terminal
workflow truth. A successful process exit is operationally misleading: a
caller can interpret the trusted-host operation as complete even though the
required terminal artifact is absent.

The corruption test helper currently accepts either a zero-exit
`terminal_replay_report_generation_failed` response or a bounded nonzero
invalid-state response. That permissiveness masks this defect.

Required fix:

1. make the host mapping return a bounded `Result` or equivalent;
2. return non-success for `GenerationFailed` and `PersistenceFailed` on fresh
   and replay paths;
3. preserve the terminal workflow run and event history unchanged;
4. require non-success in all corrupt-history fixtures; and
5. keep stdout, stderr, and errors free of report contents and identifiers.

## 9. Test Quality Assessment

The focused suite is strong for passing/failed artifacts, v2 honesty, replay,
concurrency, crash recovery, history corruption, event immutability, and fixed
output privacy.

Two focused branch proofs remain missing and should be added with the blocker
fix:

- a different valid artifact at the same run/report key returns the stable
  duplicate-conflict failure without overwrite; and
- an unreadable or invalid duplicate returns the stable duplicate-unreadable
  failure without repair.

Timed-out concrete-result artifact coverage remains a non-blocking follow-up if
the closed host runner cannot produce that posture without introducing a new
test seam.

## 10. Privacy And Redaction Assessment

The report and artifact remain reference-oriented. Fixed host output and custom
Debug formatting do not expose report IDs, run IDs, paths, report text,
citations, command output, source/spec contents, parser/provider payloads,
environment values, credentials, authorization headers, keys, tokens, or
approval reasons.

No encryption-at-rest, multi-user access-control, retention, DLP, or
regulated-data claim was introduced.

## 11. Validation Assessment

The following passed on the reviewed commit:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused trusted-host Core integrity tests, 4 tests;
- foreground host tests, 19 tests;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Opt-in live-provider tests remained ignored by design and are unrelated to this
no-provider artifact path.

## 12. Governed Review Record

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1792667631146943000-2`
- Approval ID:
  `approval/run-1792667631146943000-2/review-scope-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/8f9614325a6a0f05`
- Phase status: `Completed`.
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.

The kernel coordinated the proof-backed review gate. The agent inspected code,
tests, documentation, and validation evidence and wrote this review outside the
kernel under the approved read-only implementation scope.

## 13. Non-Blocking Follow-Ups

- Add a timed-out concrete-result artifact proof when an existing closed runner
  seam can produce it without widening production behavior.
- Consider eliminating the second post-composition run rehydration in a later
  refactor by returning the validated durable run context from one private
  composition boundary. Current terminal history is immutable through supported
  APIs, so this is not an acceptance blocker.
- Keep released report inspection/export deferred until authorization,
  retention, and sensitivity are separately designed.

## 14. Recommended Next Phase

Implement the narrow trusted-host terminal artifact failure-exit blocker fix.
Do not broaden the artifact path or add a released surface. After the fix,
perform a focused blocker-fix review before accepting the implementation.
