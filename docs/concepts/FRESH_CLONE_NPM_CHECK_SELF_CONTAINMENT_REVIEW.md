# Fresh-Clone `npm run check` Self-Containment Review

Status: accepted; reported current-main P1 disproved

## Review Scope

This focused maintainer review evaluated:

- exact-current-main binding of the isolated clone;
- clean dependency installation;
- release-only CLI build posture;
- absence of `target/debug/workflow-os` before top-level validation;
- complete pinned Node and Rust toolchain configuration;
- dogfood helper and TypeScript SDK behavior;
- disclosure of the first evaluator-environment failure;
- the decision not to change already-correct product behavior; and
- release-authority and roadmap reconciliation.

Product implementation, publication, candidate rebuild, tag creation,
provider behavior, dependency changes, and independent-assurance claims were
outside review authority.

## Evidence Rechecked

- The isolated clone was bound to
  `6468785d4cb235f80dd5c8865265119075c7ba89`, the exact current `main` at
  reproduction start.
- `npm ci` completed before validation.
- Only the release CLI was built before the self-containment replay.
- `target/debug/workflow-os` was absent immediately before top-level
  `npm run check`.
- All 36 dogfood helper tests passed without that binary.
- The TypeScript SDK suite built the debug CLI through its maintained test
  path and passed all 15 tests.
- Top-level `npm run check` exited successfully.
- Repository documentation validation and diff hygiene passed after the
  evidence reconciliation.

## Evaluator-Environment Finding

The first replay attempt placed the repository Cargo shim on `PATH` without
setting its paired `RUSTUP_HOME`. Five SDK validation cases then failed because
`rustup` could not choose a toolchain. This was not a kernel, CLI, SDK, helper,
or package-script defect.

The reproduction report correctly discloses that failed diagnostic attempt.
The exact retry used the complete pinned environment, began from the same
absent-debug-binary posture, and passed. The successful result is therefore
accepted without suppressing contrary diagnostic history.

## Finding

No P0 or P1 product defect remains in this bounded scope. The current top-level
validation command is self-contained for the reported case. A code or package-
script change would add churn without closing a reproduced failure.

The review also confirms that this report is distinct from the earlier real
dry-run-helper defect. That historical defect remains fixed and directly
covered by the helper's missing-binary and dry-run tests.

## Governed Review Phase

- Workflow: `dg/review`
- Run: `run-1793824917900574000-2`
- Approval:
  `approval/run-1793824917900574000-2/review-scope-approved`
- Presentation: `presentation/f4bf136ddac1816d`
- Approval outcome: granted under delegated maintainer authority after the
  complete review handoff was presented
- Phase status: completed
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  proof-enforced approval presentation
- Review posture: owner-conducted focused review, not independent release or
  security assurance
- Approved scope: exact-main, binary-presence, pinned-toolchain, report, and
  release-authority verification only
- Validation: clean-room top-level `npm run check`, repository
  `npm run check`, `npm run check:integrations`, `npm run check:docs`,
  `npm audit --audit-level=moderate` with zero vulnerabilities, and
  `git diff --check` passed
- Out-of-kernel work: source and documentation inspection, isolated replay
  output review, deterministic validation, and this review record

## Verdict

Accept the reproduction and no-code-change disposition. Clear the reported
fresh-clone self-containment item from the release-admission blockers. Keep
publication unauthorized: independent release/security assurance, complete
frozen-command validation, scoped candidate transfer, and an explicit
publication decision remain open.
