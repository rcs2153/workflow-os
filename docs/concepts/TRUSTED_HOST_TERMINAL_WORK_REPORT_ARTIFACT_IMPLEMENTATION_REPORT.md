# Trusted-Host Terminal WorkReport Artifact Implementation Report

## 1. Executive Summary

The unpublished trusted-host application now persists one validated terminal
`WorkReport` artifact for either accepted closed no-write profile. The path
composes existing primitives rather than adding a new model or store: exact
durable-history validation produces a deterministic private v2 report, the
existing artifact integrity gate validates it with provider integration
disabled, and the existing SQLite artifact store performs a create-only write.

Workflow terminal truth remains authoritative. Report generation, artifact
validation, storage, or duplicate reconciliation failure cannot change the run
status, append workflow events, rerun the local process, or fabricate evidence.

## 2. Scope Completed

- Moved the private report commitment and contract version from v1 to v2.
- Replaced the false in-memory-only limitation with storage-neutral disclosure.
- Added a private bounded artifact result with persisted, already-persisted,
  generation-failed, and persistence-failed postures.
- Added a private persistence helper over the existing report composer,
  artifact record, integrity gate, and SQLite stores.
- Enabled strict SideEffect citation, approval-linkage, and decision gates while
  explicitly disabling the high-assurance disclosure policy for these closed
  no-write profiles.
- Added full validated equality reconciliation for create-only duplicate writes.
- Integrated fixed artifact posture into fresh terminal and terminal replay
  paths of the unpublished local host.
- Added focused terminal, replay, recovery, corruption, privacy, and regression
  coverage.

## 3. Scope Explicitly Not Completed

This phase did not add a released CLI inspection or export surface, another
profile, arbitrary command execution, scheduling, runtime configuration,
providers, network calls, credentials, external writes, SideEffect execution,
workflow schema changes, examples, hosted expansion, OpenShell, nested
harnesses, signing, encryption, retention, DLP, or release-posture changes.

## 4. Implementation Boundary

The fixed order is:

```text
terminal trusted-host run
  -> exact durable-history validation
  -> deterministic private v2 WorkReport
  -> validated WorkReportArtifactRecord
  -> existing no-provider artifact integrity gates
  -> existing SQLite WorkReportArtifactStore
  -> bounded persisted/already-persisted/failure posture
```

`persist_trusted_host_terminal_work_report_artifact(...)` is hidden behind the
existing trusted-host application feature. It accepts only the admitted request,
closed profile, and existing SQLite backend. It does not accept report prose,
artifact identity, provider configuration, or an alternate store.

## 5. Report And Artifact Identity

The v2 report ID is derived deterministically from the run ID, immutable bundle
root hash, profile, step, command, and local-check result identifiers. The
artifact model derives and validates its metadata from that report. The helper
reconfirms artifact/run identity before attempting the existing artifact gate.

The persisted report no longer claims that it remains only in memory. It states
that report contents remain available only through private trusted-host
composition and that no released inspection or export surface exists.

## 6. Integrity And Failure Semantics

The helper invokes `write_report_artifact_with_explicit_integrations(...)` with
provider integration set to `None`, all SideEffect citation requirements
enabled, approval references required for approval-gated SideEffects, decisions
required for approved or denied SideEffects, and
`WorkReportArtifactHighAssuranceDisclosurePolicy::disabled()`.

Fresh writes are create-only. A duplicate is accepted only after reading the
exact run/report key and comparing the complete validated artifact. A missing,
unreadable, invalid, or conflicting duplicate fails closed. No overwrite or
repair path exists.

## 7. Replay And Recovery

Exact terminal replay does not rerun the process or append workflow events. It
returns `AlreadyPersisted` when the full artifact is equal. If a terminal run
exists without its artifact, replay composes and writes exactly one artifact
while preserving the event count. This covers the crash window after terminal
commit and before artifact commit. Replay after artifact commit covers the
opposite response-loss window.

## 8. Corrupt-History Protection

Full host-entry fixtures remove the original artifact, tamper the durable event
history, and then invoke terminal replay. Contradictory passed/failed result and
run truth, concrete result plus cancellation, and result/invocation event-order
corruption fail before another artifact is written. Errors and host output stay
bounded and do not echo tampered values.

## 9. Privacy And Redaction

The host emits fixed posture strings only. The private result's `Debug` output
contains closed posture, artifact presence, and optional stable error code; it
does not format report or artifact contents or identifiers. Artifact citations
remain stable-reference based. The phase does not store or print command output,
source/spec contents, parser or provider payloads, paths, environment values,
credentials, authorization headers, keys, tokens, or approval reasons.

## 10. Test Coverage

Focused tests cover:

- passing docs-check and project-validation artifact persistence;
- failed concrete-check artifact persistence with honest terminal status;
- deterministic v2 report identity and storage-neutral limitation;
- exact replay with one artifact and no additional events;
- concurrent equal terminal replays converging on one artifact;
- recovery of a missing artifact after terminal commit;
- contradictory result/run status rejection;
- cancellation contradiction rejection;
- result-to-invocation event-order rejection;
- bounded host success and failure posture;
- required-output and project-validation failure regression behavior;
- replay identity and executable substitution rejection; and
- waiting/non-terminal and unsupported-topology behavior.

## 11. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --features trusted-host-application-spi
  trusted_host_terminal`: passed, 4 focused tests.
- `cargo test -p workflow-local-host --test foreground_admission`: passed, 19 tests.
- `cargo test --workspace`: passed; opt-in live-provider tests remained ignored
  by design.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 12. Governed Phase Record

- Dogfood workflow ID: `dg/runtime-composition`
- Run ID: `run-1792666505134097000-2`
- Approval ID:
  `approval/run-1792666505134097000-2/composition-approved`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof `presentation/0efcfea223b876b1`
- Phase status: `Completed`.
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.

The kernel coordinated the proof-backed scope and approval boundary. The agent
performed repository inspection, code edits, tests, documentation, and git work
outside the kernel under that approved scope.

## 13. Remaining Limitations

- The artifact has no released inspection, rendering, or export surface.
- No store fault-injection abstraction was added; recovery is proven through
  the real SQLite boundary and explicit artifact removal.
- The trusted-host application remains unpublished, local, closed-profile, and
  no-write.
- No provider or live sandbox proof is part of this phase.

## 14. Recommended Next Phase

Perform focused maintainer/security review of this private artifact path. Review
must prioritize gate invocation, exact duplicate equality, terminal-truth
isolation, recovery, corrupt-history rejection, privacy, and the absence of any
released or provider-write broadening.
