# Fresh-Clone `npm run check` Self-Containment Reproduction Report

Status: reported current-main P1 disproved; no product change required

## Decision

The reported dependency of top-level `npm run check` on an ambient
`target/debug/workflow-os` binary does not reproduce on current `main`.

An isolated clone of exact commit
`6468785d4cb235f80dd5c8865265119075c7ba89` completed `npm ci`, built only the
release CLI, confirmed that `target/debug/workflow-os` was absent, and then
completed `npm run check`. The TypeScript SDK validation suite built the debug
CLI as part of its own test path before invoking it.

No helper, test, validation command, package script, or runtime behavior was
changed. Changing working behavior to fit an unreproduced report would weaken
the evidence boundary rather than improve it.

## Governed Blocker Phase

- Workflow: `dg/blocker`
- Run: `run-1793824622086957000-2`
- Approval: `approval/run-1793824622086957000-2/fix-approved`
- Presentation: `presentation/f26cbd4919163ad9`
- Approval outcome: granted under delegated maintainer authority after the
  complete bounded handoff was presented
- Phase status: completed
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  one persisted approval-presentation proof
- Approved scope: reproduce the report from an isolated current-main clone and
  apply the smallest validation-contract correction only if confirmed
- Strict non-goals: publication, candidate rebuild, tag creation, provider or
  kernel semantic changes, dependencies, or unrelated cleanup
- Validation: clean-clone `npm ci`, release-only CLI build, absent debug-binary
  assertion, top-level `npm run check`, repository `npm run check:docs`, and
  `git diff --check` passed
- Out-of-kernel work: isolated clone creation, local release build, filesystem
  presence check, validation commands, documentation reconciliation, and this
  report

## Clean-Room Procedure

The reproduction used a new clone under `/private/tmp` and the repository's
pinned Node and Rust toolchains. The primary checkout and its build outputs
were not used as test inputs.

1. Clone exact current `main`.
2. Run `npm ci`.
3. Run only:

   ```text
   cargo build --release --locked -p workflow-cli --bin workflow-os
   ```

4. Confirm `target/debug/workflow-os` does not exist.
5. Run top-level `npm run check` with the complete pinned `CARGO_HOME`,
   `RUSTUP_HOME`, Node, Cargo, and npm-cache environment.

The command passed. All 36 dogfood helper tests passed before the SDK suite
built the missing debug CLI. The SDK suite then passed all 15 tests, including
the Rust CLI validation cases.

## Evaluator Correction

The first local replay attempt exposed the repository's Cargo shim on `PATH`
without the corresponding pinned `RUSTUP_HOME`. Five SDK tests failed because
`rustup` could not select a toolchain. That result was an evaluator-environment
error, not a Workflow OS product failure, and is not represented as evidence
for or against self-containment.

The exact retry used the complete pinned toolchain environment, again began
without `target/debug/workflow-os`, and passed. The correction is disclosed so
the successful replay is not manufactured by omitting a failed diagnostic
attempt.

## Historical Distinction

An earlier fresh-clone defect was real: dry-run dogfood helper tests required a
non-executed binary while using `--no-build`. That defect was already corrected
before the `0.4.0-preview.1` version-alignment phase and remains covered by
focused helper tests.

This report addresses the later, distinct claim that current top-level
`npm run check` still requires an ambient debug binary after only a release
build. Current-main evidence disproves that claim.

## Boundary

This result clears only the reported fresh-clone self-containment admission
item. It does not authorize publication, replace independent release and
security assurance, transfer candidate artifacts, rerun frozen candidate
commands, classify opt-in credential tests, or authorize any external
mutation.

## Focused Review

Focused maintainer review accepts the reproduction and no-code-change decision
with no unresolved P0 or P1 finding in this bounded scope. See the [focused
review](FRESH_CLONE_NPM_CHECK_SELF_CONTAINMENT_REVIEW.md).

Independent release/security assurance, complete frozen-command validation,
scoped candidate transfer, and a separately governed publication decision
remain open.
