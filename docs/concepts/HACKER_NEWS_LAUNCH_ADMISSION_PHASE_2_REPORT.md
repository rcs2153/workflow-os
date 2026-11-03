# Hacker News Launch Admission Phase 2 Report

Status: implementation complete; focused review accepted; exact-head CI required

## Decision

Phase 2 composes the existing kernel into one real, credential-free onboarding
operation:

```text
workflow-os first-run --write-report-artifact --run-id launch-first-run-v1
```

The operation is not a mock workflow and does not introduce a generic command
runner. It validates the initialized project through the closed authoritative
project-validation profile, binds the bounded first-run analysis into immutable
run context, applies proportional governance, persists the terminal WorkReport,
and makes its evidence posture inspectable.

## Implemented Scope

- generated onboarding now recommends the real report-artifact operation;
- the mock first-run workflow remains available only as an explicitly labelled
  approval/audit demonstration;
- default and verbose first-run output recommend the same exact operation;
- the initial authoritative project-validation result is preserved as a typed
  `LocalCheckResultReference` and cited by the terminal WorkReport;
- Core exposes an additive resolved-context route result carrying the exact
  same-call validation reference, rather than requiring the CLI to reconstruct
  provenance;
- terminal artifact recovery continues to rerun the bounded validation check,
  reject changed resolved context, and cite the reassessment result;
- human and JSON `inspect` output now expose section count, total citation
  count, missing-citation count, and local-check citation count; and
- terminal `next-action` rejection remains unchanged and non-mutating.

## Correctness Finding Resolved During Implementation

The initial implementation path passed no validation reference to the report
composer, while the terminal recovery path already preserved its reassessment
reference. The first end-to-end inspect assertion exposed the mismatch as:

```text
citations=0 missing_citations=0 local_check_citations=0
```

The correction was made in Core. A new additive resolved-context routing
consumer returns both route truth and the stable reference derived from the
same authoritative local-check result and immutable declaration identity. The
CLI now consumes that reference. It does not invent a check ID, copy command
output, or create a parallel evidence model.

Focused review additionally required caller-controlled reference metadata to
fail validation before route entry. That correction preserves fail-before-
mutation behavior while keeping exact reference identity Core-derived. See the
[Phase 2 focused review](HACKER_NEWS_LAUNCH_ADMISSION_PHASE_2_REVIEW.md).

The accepted initial path now reports:

```text
citations=1 missing_citations=0 local_check_citations=1
```

## Runtime And Recovery Posture

The selected operation:

- requires the generated closed first-run contract;
- rejects mock mode and customized generated contracts;
- executes only the existing project-validation local-check profile;
- carries no provider credential or external mutation;
- writes only governed local state and the terminal WorkReport artifact;
- replays an existing complete artifact without duplicate execution or writes;
- rejects terminal recovery after bounded analysis drift;
- reruns validation before recovering a missing terminal artifact; and
- does not make `next-action` authoritative for terminal runs.

## Validation

Completed locally:

- `cargo test --workspace --all-features`: passed;
- `cargo test -p workflow-cli --test cli first_run_`: 22 passed;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed;
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps`:
  passed;
- `npm run check`: passed, including docs, dogfood helper, integration helper,
  release packaging, TypeScript, and contract checks;
- `cargo fmt --all -- --check`: passed; and
- `git diff --check`: passed.

The full suite included 176 CLI tests, 773 Core unit tests, 345 local-executor
tests, and the remaining workspace integration and documentation tests.

## Deferred Scope

Phase 2 does not authorize:

- generic local command, shell, package-manager, or build execution;
- provider fixtures, credentials, or provider mutation;
- parallel or nested runtime execution;
- artifact publication, release tags, or a Hacker News submission;
- Salsa, OpenShell activation, hosting, or enterprise administration; or
- a claim that clean-room admission has passed.

Final native candidates must be rebuilt from the exact clean post-composition
commit before Phase 3 uses them. Phase 3 must then execute this exact packaged
operation against two unrelated repositories and preserve its evidence.

## Governed Phase

- workflow: `dg/implement`
- run: `run-1793746988649952000-2`
- approval: `approval/run-1793746988649952000-2/implementation-approved`
- presentation proof: `presentation/262eaa55572aa27a`
- approval outcome: granted under delegated-maintainer authority
- event summary: 39 events, 1 approval, 0 retries, 0 escalations

Repository edits, builds, tests, inspection, and report writing were performed
outside the kernel and are disclosed here. The kernel governed scope and
approval; it did not execute shell commands or persist this report.
