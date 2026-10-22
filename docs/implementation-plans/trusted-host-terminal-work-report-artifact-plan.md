# Trusted-Host Terminal WorkReport Artifact Plan

Status: planning complete; focused maintainer/security review is required
before implementation.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Report Artifact Plan](report-artifact-plan.md)
- [Authoritative WorkReport Artifact Persistence Plan](authoritative-work-report-artifact-persistence-plan.md)
- [Trusted-Host Terminal WorkReport Composition Plan](trusted-host-terminal-work-report-composition-plan.md)
- [Trusted-Host Terminal WorkReport Composition Blocker Fix Review](../concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_COMPOSITION_BLOCKER_FIX_REVIEW.md)

## 1. Executive Summary

The unpublished trusted-host application can execute two closed no-write
profiles and compose a validated in-memory terminal `WorkReport` from exact
durable workflow history. Workflow OS also already has a validated
`WorkReportArtifactRecord`, a `WorkReportArtifactStore`, SQLite artifact
storage, generic artifact integrity gates, and reviewed exact-duplicate
reconciliation patterns.

The next implementation should compose those accepted primitives into one
private trusted-host artifact path. After the run is terminally committed, Core
must rehydrate and validate the exact history, generate the deterministic
report, construct a validated artifact, pass the existing no-provider artifact
gates, and write through the existing SQLite artifact store. Artifact failure
must remain separate from workflow pass/fail truth.

This plan does not implement anything. It does not authorize a released CLI
surface, public report export, a new profile, arbitrary commands, scheduling,
provider access, external writes, hosted persistence, workflow schema changes,
examples, or nested harness execution.

## 2. Planning Decision

Implement one explicit private composition boundary rather than another
artifact model or store:

```text
terminal trusted-host run
  -> exact durable-history validation
  -> deterministic v2 private WorkReport
  -> validated WorkReportArtifactRecord
  -> existing no-provider artifact integrity gates
  -> existing SQLite WorkReportArtifactStore
  -> bounded persisted/already-persisted/failure posture
```

The first artifact write is automatic only inside the unpublished trusted-host
operation after one of its two closed profiles reaches a concrete terminal
result. It is not automatic artifact generation for ordinary executor paths.

## 3. Goals

- Persist the accepted trusted-host terminal report through the existing local
  SQLite artifact store.
- Bind the artifact to the exact run, immutable bundle, result event,
  invocation terminal event, run terminal event, profile, command, and report
  identity already validated by the composer.
- Reuse existing artifact models, stores, and integrity gates.
- Preserve workflow terminal truth when report generation or artifact writing
  fails.
- Reconcile exact replay without rerunning the process or appending events.
- Recover safely from a crash after run terminalization and before or after the
  artifact commit.
- Expose only fixed artifact posture from the unpublished local-host binary.
- Keep report and artifact contents payload-free and redaction-safe.

## 4. Non-Goals

The implementation must not add:

- a new report, artifact, or artifact-store model;
- a new database table or state-backend capability;
- automatic artifacts for ordinary `LocalExecutor` paths;
- report artifacts for waiting, blocked, denied-before-run, or prepared runs;
- post-terminal workflow events or snapshot mutation;
- released `workflow-os` CLI inspection, rendering, or export;
- report body output from `workflow-local-host`;
- a third local-check profile or arbitrary command execution;
- scheduler, daemon, or runtime configuration;
- provider, network, credential, or external-write behavior;
- SideEffect execution or fabricated SideEffect records;
- workflow-declared report contracts or schema changes;
- examples, SDK changes, hosted behavior, OpenShell, or nested harnesses;
- signing, notarization, encryption-at-rest claims, DLP, retention, or access
  control; or
- release-posture changes.

## 5. Existing Foundation

The implementation should reuse:

- `compose_trusted_host_terminal_work_report(...)` for exact rehydration and
  terminal-history validation;
- `WorkReportArtifactRecord::new(...)` for model and metadata validation;
- `WorkReportArtifactStore` and its `SqliteStateBackend` implementation;
- `SideEffectRecordStore` on the same backend;
- `write_report_artifact_with_explicit_integrations(...)` with provider
  integration set to `None` and strict citation/linkage requirements;
- the create-only duplicate behavior of SQLite artifact writes; and
- the reviewed read-after-duplicate equality pattern used by authoritative
  artifact persistence.

No new dependency is justified.

## 6. Required Report Contract Correction

The current private v1 report contains this limitation:

```text
Report remains in memory and is not persisted or written as an artifact.
```

Persisting that report would create a false artifact. The implementation must
not sanitize or mutate the report after construction. Instead, it should move
the private composer to a v2 report commitment and contract version before the
first artifact write.

The v2 report should use storage-neutral bounded disclosure, for example:

```text
Report contents are available only through private trusted-host composition;
no released inspection or export surface exists.
```

Requirements:

- change the report-material domain separator from v1 to v2;
- change the private `WorkReportContractVersion` from `v1` to `v2`;
- derive a new deterministic report ID from the v2 material;
- keep the contract ID private and unchanged unless review finds ambiguity;
- never create both v1 and v2 artifacts for one operation; and
- test that the persisted report contains no false in-memory-only claim.

No v1 artifact migration is required because this trusted-host path has not
previously persisted artifacts. The v1 in-memory report remains historical
preview behavior, not a public compatibility contract.

## 7. Candidate Private API

The smallest Core surface is a hidden private-result helper, conceptually:

```rust
persist_trusted_host_terminal_work_report(
    backend: &SqliteStateBackend,
    profile: &ResolvedExplicitLocalCheckProfile,
    request: &TrustedHostLocalApplicationAdmissionRequest,
) -> TrustedHostTerminalWorkReportArtifactResult
```

The exact name should follow local conventions. The result should expose only
read-only private accessors for:

- artifact posture;
- optional validated artifact;
- optional stable error; and
- report posture when distinguishing generation from persistence failure is
  necessary.

Candidate closed artifact postures:

- `Persisted`;
- `AlreadyPersisted`;
- `GenerationFailed`; and
- `PersistenceFailed`.

Waiting, blocked, and denied-before-run outcomes do not call this helper and
therefore do not need artifact postures.

Debug output must show only closed posture, artifact presence, and an optional
error code. It must redact report and artifact contents and all identifiers.

## 8. Composition And Gate Order

The order must be fixed:

1. Resolve the explicit run ID from the admitted request.
2. Rehydrate the terminal run from SQLite.
3. Reapply all accepted trusted-host report bindings, including the strict
   result/invocation/run status matrix and event order.
4. Generate the deterministic v2 `WorkReport`.
5. Construct and validate `WorkReportArtifactRecord`.
6. Reconfirm artifact/run identity.
7. Call the existing explicit artifact integration gate with:
   - no provider integration;
   - all SideEffect citations required when any exist;
   - approval references required for any approval-gated SideEffect;
   - decisions required for approved or denied SideEffects; and
   - `WorkReportArtifactHighAssuranceDisclosurePolicy::disabled()` for these
     closed no-write profiles.
8. Write through `WorkReportArtifactStore`.
9. Reconcile a duplicate by reading the exact run/report key and requiring
   full validated artifact equality.

The helper must not bypass the generic gate merely because current reports
contain no SideEffect citations. This keeps the boundary fail-closed if the
private report evolves later.

## 9. Atomicity And Failure Semantics

Run terminalization and artifact persistence are intentionally separate
transactions.

- The run, invocation, and local-check result commit remains authoritative.
- Artifact failure must not change run status, append an event, mutate a
  snapshot, or rerun the process.
- A report-generation failure writes no artifact.
- An artifact-validation or gate failure writes no artifact.
- A store failure returns `PersistenceFailed` with a stable non-leaking code.
- The unpublished host process should return a bounded non-success result for
  generation or persistence failure while preserving terminal workflow truth.
- It must not print quiet-success or persisted wording after failure.

This is a failed handoff/artifact obligation, not a retroactive workflow
failure.

## 10. Replay And Crash Recovery

Exact replay must remain process-free and event-free.

- If the run is terminal and no artifact exists, compose and write it.
- If an exactly equal validated artifact already exists, return
  `AlreadyPersisted`.
- If a duplicate write races with another caller, read the committed artifact
  and accept only exact equality.
- If the same run/report identity contains different content, fail closed with
  a stable conflict code.
- If a duplicate cannot be read or validated, fail closed; do not overwrite or
  repair it.

Crash windows:

- after terminal run commit but before artifact write: replay completes the
  artifact without process execution;
- after artifact commit but before caller response: replay returns
  `AlreadyPersisted`;
- during SQLite artifact write: SQLite transaction semantics decide whether a
  record exists, and replay follows the two rules above.

## 11. Host Integration

Replace the current private host `report_posture(...)` composition call with
the reviewed artifact helper after implementation acceptance.

Fixed output may distinguish:

- fresh terminal artifact persisted;
- terminal replay artifact already persisted;
- report generation failed; and
- artifact persistence failed.

Output must not include report IDs, run IDs, paths, report text, citations,
error messages, or artifact contents. This remains an unpublished
`workflow-local-host` binary surface and does not authorize changes to the
released `workflow-os` CLI.

## 12. Corrupt-History Requirement

The composition blocker-fix review retained one non-blocking follow-up before
persistence widened the trust boundary. It becomes required in this phase.

Tests must exercise fully rehydrated tampered histories through the report or
artifact composition entry point for:

- passed result with failed invocation or run;
- failed/timed-out result with successful invocation or completed run;
- concrete result with cancellation;
- invocation terminal before or equal to the result event; and
- run terminal before or equal to the invocation terminal event.

Each must fail before artifact writing with stable non-leaking errors.

## 13. Privacy And Redaction

The artifact must never store or print:

- stdout, stderr, command transcripts, or local-check summaries;
- source, spec, parser, provider, network, or response payloads;
- project, executable, source, or artifact-store paths;
- environment values, credentials, authorization headers, keys, or tokens;
- approval reasons or policy source text; or
- caller-authored unbounded report prose.

Artifact storage inherits the current local-preview security posture. This
phase does not claim encryption at rest, multi-user authorization, retention,
or regulated-data suitability.

## 14. Tests

Focused implementation tests must prove:

1. passing docs-check and project-validation runs persist one artifact;
2. failed concrete checks persist an honest failed terminal artifact;
3. timed-out concrete results are representable and persist only with failed
   invocation/run truth;
4. waiting, blocked, prepared, and denied-before-run outcomes write nothing;
5. artifact metadata matches the exact report and run identity;
6. the v2 report contains no false in-memory-only limitation;
7. the artifact gate runs with no provider integration;
8. report generation failure writes nothing and preserves terminal truth;
9. artifact failure preserves terminal truth and event history;
10. exact replay reruns no process and appends no events;
11. exact replay returns `AlreadyPersisted` for an equal artifact;
12. concurrent equal callers converge on one artifact;
13. conflicting duplicate content fails closed;
14. unreadable or corrupt duplicate content fails closed;
15. crash after terminal commit can be recovered by replay;
16. crash after artifact commit can be reconciled by replay;
17. all corrupt-history cases in Section 12 fail before any write;
18. fixed host output discloses posture without identifiers or content;
19. Debug and errors remain payload-free;
20. no post-terminal workflow event or snapshot mutation occurs;
21. no provider, network, credential, or external-write path is invoked;
22. existing in-memory report, artifact-store, trusted-host, and foreground
    host tests remain green; and
23. the complete workspace suite passes.

## 15. Validation

The implementation phase should run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused Core artifact/composition tests with the trusted-host feature;
- `cargo test -p workflow-local-host --test foreground_admission`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Any skipped opt-in live-provider proof must be disclosed and remains unrelated
to this no-provider artifact phase.

## 16. Proposed Implementation Sequence

1. Review this plan before code changes.
2. Move private report commitment and contract version to v2 with
   storage-neutral limitations.
3. Add the private bounded artifact result and explicit persistence helper.
4. Reuse the existing artifact gates and SQLite store.
5. Add exact duplicate reconciliation.
6. Integrate the helper into fresh terminal and terminal-replay host paths.
7. Add corrupt-history, recovery, concurrency, privacy, and regression tests.
8. Run full repository validation.
9. Perform focused maintainer/security review before considering any released
   inspection or export surface.

## 17. Open Questions Resolved For V1

- **Store:** use the existing `SqliteStateBackend`; add no store or schema.
- **Artifact identity:** reuse deterministic v2 `WorkReportId`.
- **Duplicate semantics:** create-only write plus exact read-after-duplicate
  equality.
- **Workflow semantics:** artifact failure never rewrites terminal run truth.
- **Events:** append no report-created or artifact-created workflow event.
- **Configuration:** add no flag, schema field, or runtime config.
- **Provider posture:** no provider integration and no external write.
- **User surface:** fixed unpublished host posture only.

## 18. Remaining Open Questions

- Should artifact persistence failure use one private result error namespace or
  preserve the underlying stable artifact-store code in the private result?
- Should fresh persistence and exact replay share one host success string, or
  disclose `persisted` versus `already_persisted`?
- Is a focused SQLite fault-injection seam already sufficient for both crash
  windows, or should implementation add one test-only boundary?

These are implementation details for review. They do not authorize a public
artifact API or broader runtime behavior.

## 19. Final Recommendation

After focused plan review, implement one private, integrated trusted-host
terminal WorkReport artifact vertical slice using existing models, gates, and
SQLite storage. Do not create another primitive family.

The implementation must first correct the report's storage claim, then prove
exact replay, corrupt-history rejection, and failure isolation. Released CLI
inspection/export, another profile, arbitrary commands, scheduling, providers,
external writes, hosted persistence, schemas, examples, and nested harnesses
remain explicitly deferred.

## 20. Governed Planning Record

- Dogfood workflow ID: `dg/d`
- Run ID: `run-1792666274481624000-2`
- Approval ID: `approval/run-1792666274481624000-2/planning-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/ad0c65db2725f9ba`
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.
- Validation summary: documentation and diff checks passed.

The agent inspected repository contracts and wrote planning documentation
outside the kernel under the approved scope. The kernel coordinated the
proof-backed planning gate and durable phase record; it did not inspect code,
write files, run documentation checks, commit changes, or perform provider
activity.
