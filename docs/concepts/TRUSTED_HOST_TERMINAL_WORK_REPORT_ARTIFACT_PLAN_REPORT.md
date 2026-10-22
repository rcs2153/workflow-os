# Trusted-Host Terminal WorkReport Artifact Planning Report

## 1. Executive Summary

Planning is complete for the smallest private trusted-host terminal
`WorkReport` artifact boundary. The plan composes accepted report generation,
artifact models, integrity gates, SQLite storage, and replay semantics without
adding another primitive family.

## 2. Scope Completed

- Inspected accepted trusted-host report composition and blocker-fix reviews.
- Inspected artifact models, stores, integrity gates, and authoritative
  persistence precedent.
- Defined one private integrated artifact vertical slice.
- Defined exact duplicate, crash recovery, workflow-failure isolation,
  privacy, and corrupt-history requirements.
- Updated the active roadmap queue.

## 3. Scope Explicitly Not Completed

No Rust code, artifact write, CLI behavior, profile, command, scheduler,
provider, external write, hosted persistence, schema, example, or nested
harness behavior was implemented.

## 4. Key Planning Decision

Reuse the existing `WorkReportArtifactRecord`, artifact integrity gates,
`WorkReportArtifactStore`, and `SqliteStateBackend`. The future helper should
compose and persist only after exact terminal rehydration, then reconcile
create-only duplicates by full validated equality.

## 5. Important Finding

The current private v1 report says it remains in memory and is not persisted.
Persisting it unchanged would produce a false artifact. The plan therefore
requires a deterministic v2 private report commitment with storage-neutral
limitations before the first write.

## 6. Safety Posture

Artifact persistence remains separate from workflow terminal truth. Failure
must not append events, mutate snapshots, rerun the process, or call a provider.
The previously non-blocking corrupt-history fixture follow-up becomes mandatory
before persistence broadens the trust boundary.

## 7. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 8. Governed Phase Record

- Workflow ID: `dg/d`
- Run ID: `run-1792666274481624000-2`
- Approval ID: `approval/run-1792666274481624000-2/planning-approved`
- Approval outcome: granted with persisted presentation proof
  `presentation/ad0c65db2725f9ba`
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.

Repository inspection and documentation edits occurred outside the kernel.
The kernel coordinated governance only.

## 9. Remaining Limitations

This is planning only. The trusted-host report remains in memory, no private
artifact is written by this path, and no released inspection/export surface is
authorized.

## 10. Recommended Next Phase

Perform focused maintainer/security review of the artifact plan. If accepted,
implement the private integrated artifact vertical slice without broadening
profiles, providers, external writes, hosted behavior, or released CLI
surfaces.
