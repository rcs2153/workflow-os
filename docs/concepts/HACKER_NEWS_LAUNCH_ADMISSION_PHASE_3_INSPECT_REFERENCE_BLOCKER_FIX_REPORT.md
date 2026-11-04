# Hacker News Launch Admission Phase 3 Inspect Reference Blocker Fix Report

Status: blocker corrected, focused review accepted, and local validation
complete; exact-head merge and final clean-room rerun required

## Clean-Room Artifact

The first Phase 3 pass built an optimized Apple Silicon binary from a separate
clean clone at exact merged commit:

```text
92c31692c0b56790ee31739d48f56a8891e3e4ce
```

The non-public package contract produced:

```text
archive: workflow-os-v0.3.0-preview.1-aarch64-apple-darwin.tar.gz
archive_sha256: 27516182f72bc3fa644f7a17f0d292eb3d2fdee347f391d34f8e69d39cd1a65a
binary_sha256: 75dd46a4b3b81106214b905adcb9029a7791a0dfefc821aaf161145441b38074
source_commit: 92c31692c0b56790ee31739d48f56a8891e3e4ce
working_tree_clean: true
provisional: true
```

The checksum, target, version, install, overwrite, uninstall, and state
preservation contracts passed. The package remained explicitly provisional and
was not uploaded or published. The exact merged-commit Linux packaging job also
passed in GitHub Actions run `41741617610`; the complete main-branch run remains
a separate terminal-green requirement.

## First-Pass Repositories

Two fresh unrelated disposable Git repositories were created outside the
Workflow OS checkout:

- a TypeScript-shaped repository with `package.json`, source/test layout, and
  existing `AGENTS.md`; and
- a Rust-shaped repository with `Cargo.toml`, source layout, and existing
  `AGENTS.md`.

Only the installed packaged binary was used for the user path. Both repositories
completed dry-run scaffold, scaffold, validation, default first-run, the exact
report-artifact command, status, human inspect, JSON inspect, and exact replay
in under one second of local CLI wall time, well below the ten-minute admission
threshold.

Both paths:

- preserved unmanaged agent guidance and appended the managed block;
- detected ecosystem-specific metadata and recommendations;
- used no mock handlers, provider access, external writes, or approval prompts;
- completed with ten durable events, zero approvals, retries, or escalations;
- persisted one eleven-section WorkReport with one non-missing local-check
  citation;
- replayed the same run ID as `existing_terminal` without new events or
  duplicate artifacts; and
- returned the documented `executor.governed_continuation.run.not_running`
  diagnostic for terminal `next-action`.

The operation validated the Workflow OS governance project. It did not run the
repositories' package-manager, compiler, or test commands. Those checks remain
review-only recommendations and are not claimed as execution evidence.

## Recovery Evidence

A bounded OS-level permission fault made only the WorkReport directory
temporarily unwritable for a new Rust evaluation run. No runtime record was
deleted or edited.

The run reached terminal `Completed` with the expected ten-event trail, then
failed honestly with:

```text
work_report_artifact.authoritative.write_failed
```

After removing the permission fault, repeating the same packaged command and
run ID preserved the original events, reran authoritative validation against
unchanged context, and persisted exactly one cited WorkReport. This proves the
reviewed terminal-run/missing-artifact recovery gap without manual state repair.

Uninstall then removed only the isolated binary. TypeScript state remained at
32 files and Rust state at 59 files before and after uninstall.

## Admission Blocker

The first pass exposed:

```text
citations=1 missing_citations=0 local_check_citations=1
```

That proves citation posture, but the launch plan requires the stable
project-validation citation itself to be visible through bounded `inspect`
output. A count is not independently checkable identity.

The correction adds only:

- one `local_check_reference=<validated-reference>` line per report artifact in
  human inspect output; and
- a deterministic `local_check_references` string array in JSON inspect output.

References are collected from existing `LocalCheckResult` citation targets,
sorted, and deduplicated. `WorkReportStableReference` construction already
rejects empty, unbounded, and secret-like values. The projection does not expose
report text, command output, repository content, redaction values, or evidence
payloads.

Focused review found that bounded reference validation does not prohibit every
control character. The final local correction therefore escapes references for
single-record terminal rendering and extends the shared JSON escaper to cover
quotes, backslashes, named JSON controls, and every remaining U+0000-U+001F
character. A direct unit regression proves both output forms. Ordinary stable
references retain their existing human-readable form.

Reading the preserved clean-room artifact through the corrected binary produced:

```text
local_check_reference=authoritative-check/725fb9a4adad39155a66732bdad92177403bd8aeba9087a8aebf931086c0c852
```

The original ten-event trail and artifact remained unchanged.

## Validation

Completed locally:

- focused inspect-reference regression: passed;
- all 22 `first_run_` CLI tests: passed;
- `npm run check`: passed;
- `cargo fmt --all -- --check`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

Also completed before the focused-review correction:

- full workspace tests with all features: passed;
- warnings-denied workspace rustdoc: passed; and
- warnings-denied Clippy identified and drove a behavior-preserving rendering
  helper extraction, then passed.

After the final control-character correction, the complete workspace matrix,
warnings-denied Clippy, warnings-denied rustdoc, focused regressions, formatting,
docs, and diff checks passed. Exact-head CI remains required before merge.

## Governed Phase

- workflow: `dg/implement`
- run: `run-1793749611286304000-2`
- approval: `approval/run-1793749611286304000-2/implementation-approved`
- presentation proof: `presentation/9733df61796cdb80`
- outcome: granted under delegated-maintainer authority
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record

The kernel governed scope and approval. Packaging, temporary repository
creation, shell commands, OS-level fault injection, evaluation, source edits,
tests, and report writing were performed outside the kernel and are disclosed
here.

Focused review is recorded in [Hacker News Launch Admission Phase 3 Inspect
Reference Blocker Fix
Review](HACKER_NEWS_LAUNCH_ADMISSION_PHASE_3_INSPECT_REFERENCE_BLOCKER_FIX_REVIEW.md).
