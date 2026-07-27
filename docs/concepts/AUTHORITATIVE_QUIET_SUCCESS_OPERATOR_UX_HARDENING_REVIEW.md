# Authoritative Quiet-Success Operator UX Hardening Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The implementation delivers the first quiet-success operator surface without
weakening governance. It changes human presentation only after Core selected
`QuietProceed`, the durable run completed, and the in-memory WorkReport was
generated successfully.

All postures that require operator attention remain detailed. The new
`run --verbose` option and existing JSON output preserve bounded drill-down.

## 2. Scope Verification

The phase stayed within its approved presentation-only scope.

It added:

- concise human output for successful authoritative quiet runs;
- command-local `run --verbose` for bounded authoritative detail;
- focused CLI regressions;
- user-facing documentation; and
- an implementation report.

It did not add or change:

- proportional-governance assessment or route selection;
- workflow execution, retry, approval, denial, or resume semantics;
- automatic or delegated approval;
- report persistence, artifacts, or export;
- provider or OpenShell integration;
- SideEffect execution or external writes;
- schema, scaffold, hosted, enterprise, or release behavior.

## 3. Quiet-Success Eligibility Assessment

The renderer requires all of the following before using concise output:

1. the authoritative route is `QuietProceed`;
2. the durable run status is `Completed`; and
3. report posture is `Generated`.

This is the correct conservative conjunction. A quiet route alone cannot hide
a failed run or failed report, and the renderer cannot independently infer
risk or downgrade a Core-owned route.

## 4. Human Output Assessment

The concise default retains:

- terminal success;
- the fixed `quiet_success` governance label;
- durable run identity; and
- the exact inspect command.

It omits route internals, report identity, and local-check reference identity
from the default success path. Those fields remain available through
`run --verbose` and JSON.

The result meets the quiet-success product invariant: successful low-friction
work is quiet without becoming uninspectable.

## 5. Explicit And Failure Output Assessment

The detailed renderer remains active for:

- visible disclosure;
- approval-required routes;
- denial;
- failed or non-completed runs;
- report-generation failure; and
- explicit `run --verbose`.

This prevents the UX change from concealing incomplete governance or degraded
evidence/report posture.

## 6. Parser And Compatibility Assessment

`--verbose` is accepted only for authoritative execution activated by the
project declaration or the experimental compatibility flag. It does not
activate authoritative execution or change ordinary `run` semantics.

Using `run --verbose` on the ordinary path fails before state creation with a
bounded usage error. Existing global JSON behavior is unchanged.

## 7. Privacy And Redaction Assessment

Concise output contains no:

- report body or section text;
- source or spec content;
- command or provider output;
- filesystem path;
- environment value;
- credential or token;
- approval reason; or
- local-check payload.

Verbose and JSON paths retain their previously reviewed bounded,
payload-free shapes. The new renderer introduces no new caller-controlled text
surface.

## 8. Test Quality Assessment

Focused tests prove:

- concise default output for an explicit authoritative quiet run;
- concise default output for project-declared activation;
- bounded verbose route, disclosure, report, and check-reference detail;
- rejection of non-authoritative `run --verbose` before state creation;
- continued absence of report artifacts; and
- existing JSON, approval, denial, failure, executor, report, and immutable-run
  behavior through the workspace suite.

The assertions verify both required fields and fields that must not appear in
the quiet default, which is important for preventing accidental UX regression.

## 9. Documentation Assessment

The roadmap, proportional-governance plan, CLI references, current product
contract, implementation report, and this review agree that:

- quiet success is concise only after successful authoritative completion;
- explicit detail remains available;
- warning and blocking postures remain visible;
- reports remain in memory; and
- providers, OpenShell, SideEffect execution, writes, hosted controls, and
  broader profile families remain unsupported.

## 10. External User Feedback Assessment

The fresh-pull user review supports this phase directly. It finds that current
main is coherent and honest, that first-run explanation is no longer the
primary problem, and that reducing low-risk ceremony while preserving evidence
is now the most valuable product direction.

Two separate usability issues remain:

- the integration check has an opaque failure posture under Node 24 while the
  documented Node 20 path passes; and
- pre-scaffold validation repeats the missing-manifest diagnostic.

Neither issue changes this phase's runtime or presentation contract. They
should be handled as bounded CLI/tooling follow-ups rather than folded into
proportional-governance semantics.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Track and harden the Node 24 integration-check failure surface while keeping
  Node 20 as the supported repository toolchain.
- Deduplicate the pre-scaffold missing-manifest diagnostic.
- Measure whether the four-line quiet output is sufficient for real operators
  before adding more default fields.
- Keep future live disclosure projection separate from the governance
  obligation selected by Core.
- Do not begin provider or OpenShell integration before the immutable-input
  and scoped-runtime-authority sequence permits it.

## 13. Recommended Next Phase

Return to current `main` after merge and select the next incomplete
runtime-composition phase. Prefer a phase that strengthens immutable execution,
evidence completeness, or scoped authority without reintroducing low-risk
operator ceremony.

OpenShell, if pursued, should remain an optional execution provider behind a
separate threat model and adapter contract. It should not become the next
phase solely because quiet-success presentation is accepted.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1785186199070174000-2`
- approval:
  `approval/run-1785186199070174000-2/review-scope-approved`
- presentation: `presentation/00f0f10b8c8ad068`
- approval outcome: granted by delegated maintainer through
  presentation-proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation: focused authoritative CLI tests,
  `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `npm run check`, `npm run check:docs`, and
  `git diff --check` passed
- skipped checks: opt-in live adapter and provider smoke tests remained skipped
  by their existing environment-gated contracts
- report posture: the implementation report and this review are persisted in
  the repository; no runtime WorkReport artifact was generated
- out-of-kernel work: diff inspection, review authoring, validation, git, and
  pull-request operations
- kernel boundary: the kernel governed scope and approval; it did not inspect
  code, edit files, execute validation, or perform git and pull-request actions

## 15. CI Fix-Forward Review

Rust 1.97.1 CI found a mechanical Clippy blocker after this review: the
authoritative renderer reached 102 lines. A focused governed blocker phase
extracted the already-reviewed quiet-success branch into a private helper. It
did not change predicates, output, or runtime semantics.

The same focused rerun exposed an incorrect new test assumption about the
report ID prefix. The test now follows the authoritative generator's stable
`report/<run-id>` contract. These fixes do not change the phase verdict.
The completed blocker run recorded 39 events, one approval, zero retries, and
zero escalations. Focused authoritative CLI tests, workspace formatting,
workspace Clippy with warnings denied, the full Rust workspace test suite,
`npm run check`, and `git diff --check` passed after the fix.
