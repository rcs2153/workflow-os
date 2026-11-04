# Hacker News Launch Admission Phase 3 Clean-Room Report

Status: clean-room evaluation complete; focused review and exact-head CI required

## Package Identity

The final Phase 3 evaluation used a native Apple Silicon package built in a
separate clean clone from exact merged commit:

```text
c91228dcf6e7f6a8b6f92654b940efb72a11cedf
```

The provisional, non-public package recorded:

```text
archive: workflow-os-v0.3.0-preview.1-aarch64-apple-darwin.tar.gz
archive_sha256: f6df8606a525da4a54147bb3dd24d451fa346543c9f36a486a3ab80f78ddfbee
binary_sha256: 9562a076713ab79ee3a8dab79226882d66b916d49f2c3e1e5d87220d3515ec85
source_commit: c91228dcf6e7f6a8b6f92654b940efb72a11cedf
working_tree_clean: true
provisional: true
```

The checksum verified before extraction. The isolated installer installed only
the packaged binary, and the installed binary reported
`workflow-os 0.3.0-preview.1`. No artifact was uploaded or published.

Exact merged-commit GitHub Actions run `41747008662` independently rebuilt the
native Linux x86-64 candidate. Its Release Packaging Rehearsal completed the
release-contract tests, musl toolchain installation, release build, provisional
archive package, checksum verification, isolated install, and uninstall
successfully. The overall exact-commit run was still in progress at focused
review time and remains an acceptance gate.

## Clean-Room Repositories

The installed packaged binary was evaluated against two new disposable Git
repositories outside the Workflow OS checkout:

- a TypeScript-shaped repository containing existing `AGENTS.md`,
  `package.json`, `tsconfig.json`, source, and test markers; and
- a Rust-shaped repository containing existing `AGENTS.md`, `Cargo.toml`, and
  a source marker.

Neither repository was used to design the implementation. The evaluator did
not run package-manager, compiler, linter, test, or repository-specific
commands. Safe metadata informed recommendations only.

Both repositories completed:

1. governance scaffold dry-run;
2. governance initialization;
3. project validation;
4. bounded default first-run analysis;
5. the exact non-mock first-run report-artifact operation;
6. status, human inspect, and JSON inspect;
7. exact terminal replay; and
8. uninstall state-preservation verification.

The scaffold preserved each repository's existing agent instructions and
appended the managed Workflow OS block. Default first-run output detected the
correct ecosystem and recommended the real artifact command rather than the
mock demonstration.

## Evidence And Replay

The TypeScript run `phase3-typescript-final` and Rust run
`phase3-rust-final` each:

- selected `quiet_proceed` with no unexplained approval;
- completed with ten events, zero approvals, zero retries, and zero
  escalations;
- persisted exactly one eleven-section WorkReport;
- cited exactly one non-missing authoritative local-check result;
- exposed the same stable citation reference in human and JSON inspect output;
  and
- replayed as `existing_terminal` and `already_persisted` without appending an
  event or duplicating the artifact.

The stable inspect references were:

```text
TypeScript: authoritative-check/3001bed8b0548d8e68dfcaf60c5ab6b90445372bd2919777f5c52e7191f92b15
Rust:       authoritative-check/f7a538c17763abe4503ea043aa4c25bf04092e2e4b987d937968fd13b5ca355a
```

Human output remained single-record-safe, and JSON emitted a deterministic
`local_check_references` array. Neither surface exposed command output,
repository source, report bodies, or evidence payloads.

## Terminal Artifact Recovery

A second Rust run, `phase3-rust-recovery`, faulted only the local WorkReport
store by making its directory temporarily unwritable. No state record was
deleted or edited.

The governed operation completed its original ten-event run, then failed
truthfully at artifact persistence with:

```text
work_report_artifact.authoritative.write_failed
```

Inspection showed terminal `Completed`, ten events, and zero report artifacts.
After restoring filesystem access, repeating the exact command and run ID:

- preserved the original ten events;
- reran authoritative validation against unchanged governed context;
- persisted exactly one eleven-section cited WorkReport; and
- exposed stable reference
  `authoritative-check/07d860739e32d153ffd1747d8907882fc2fc1f0b4fa4b8fa0e677ccf568ead08`.

This proves the reviewed terminal-run/missing-artifact recovery boundary
without manual runtime-state repair.

## Uninstall And State Preservation

The isolated uninstall removed the installed executable and left repository
state unchanged:

```text
TypeScript state files: 36 before, 36 after
Rust state files:       63 before, 63 after
```

The package source clone, archive, extracted package, installed binary, and
evaluation repositories remained isolated under one temporary root. The
Workflow OS source checkout was not used as the evaluator's runtime or project.

## Admission Assessment

Every Phase 3 admission threshold passed locally:

- no source compilation by the evaluator after package installation;
- no provider credential, network call, fixture adapter, or mock handler;
- no silent repository-file replacement;
- no manual state edit;
- no unexplained approval prompt;
- one evidence-backed WorkReport per primary run;
- bounded independently checkable citation identity in human and JSON inspect;
- complete event history;
- exact replay without duplicate work; and
- honest recoverable artifact-write interruption.

Phase 3 introduces no new runtime behavior. Acceptance still requires focused
maintainer/product/security review and terminal-green exact-head CI for the
documentation commit. Phase 4 remains the release-candidate review; publication
requires a separate governed decision.

## Validation Summary

- clean source checkout identity: exact and clean;
- native package target and version verification: passed;
- archive SHA-256 verification: passed;
- isolated install and version check: passed;
- TypeScript scaffold, validation, first-run, artifact, inspect, and replay:
  passed;
- Rust scaffold, validation, first-run, artifact, inspect, and replay: passed;
- terminal artifact-write recovery without state repair: passed;
- uninstall and state-preservation checks: passed;
- exact merged-commit Linux packaging rehearsal: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## Governed Phase

- workflow: `dg/implement`
- run: `run-1793752139061145000-2`
- approval: `approval/run-1793752139061145000-2/implementation-approved`
- presentation proof: `presentation/07d1b48156e3548d`
- outcome: granted under delegated-maintainer authority
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record

Package construction, temporary repository creation, shell commands,
filesystem fault injection, inspection, uninstall, and report writing occurred
outside the kernel and are disclosed here. The kernel governed scope and
approval; it did not execute those operations.
