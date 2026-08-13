# Authoritative Docs-Check Profile Runtime Composition Report

## 1. Executive Summary

Workflow OS now exposes the existing canonical `DocsCheck` as the second closed
explicit authoritative local-check profile. Callers can resolve it from
explicit local paths and route it through the established immutable-run,
same-call local-check attestation, and proportional-governance executor path.

This is a bounded runtime-composition phase. It does not add command discovery,
ambient registration, project-level activation, CLI defaults, provider calls,
writes, or new workflow spec fields.

## 2. Scope Completed

- Added serialized profile vocabulary for the closed `docs_check` profile.
- Added explicit docs-check resolution with production and injected-runner APIs.
- Generalized resolved profile storage across the two canonical handler types.
- Exposed the existing explicit-facts authoritative route and request.
- Preserved registration collision checks and empty default registry behavior.
- Added focused profile and end-to-end executor tests.
- Updated the authoritative roadmap queue.

## 3. Scope Explicitly Not Completed

- No arbitrary command or shell-string registration.
- No default or ambient handler registration.
- No project-level authoritative activation for `DocsCheck`.
- No CLI default or command behavior change.
- No automatic local check execution.
- No provider, OpenShell, SideEffect, or write behavior.
- No new workflow spec field or release posture change.

## 4. Contract And Runtime Boundary

`ExplicitLocalCheckProfileId` now has two closed values:

- `workflow_os_project_validation`;
- `docs_check`.

The docs profile accepts an explicit npm executable, repository root, optional
cache path, and canonical process runner. Resolution validates these inputs but
does not execute. Execution remains a separate explicit call or occurs through
the explicit-facts authoritative route.

The public route still accepts no executable, arguments, shell text, or handler
implementation. It receives only a previously resolved closed profile. The
existing project-declared activation remains restricted to project validation.

## 5. Governance And Evidence Behavior

The second profile reuses the existing runtime path that:

- builds and claims an immutable run bundle;
- binds the exact canonical local-check declaration and contract fingerprint;
- runs the selected check exactly once in the fresh call;
- converts the accepted attestation into the selected step's evidence/check fact;
- combines that fact with caller-supplied explicit runtime facts;
- derives a complete proportional-governance binding;
- routes quiet, visible, approval, or denial through existing semantics; and
- preserves workflow state and event history through the ordinary executor.

The phase does not claim that all engineering checks are independently executed.
It proves one additional real, fixed check profile.

## 6. Privacy And Security

The profile preserves the existing bounded/redacted output model, sanitized
environment, disabled-network contract, no-source-write classification, timeout,
and path-redacting `Debug` implementations. Resolver and registration errors use
stable codes and do not echo paths or command text.

## 7. Tests Added

- Docs profile resolution does not execute the process.
- Explicit profile execution uses the canonical `npm run check:docs` contract.
- Resolver/profile mismatches fail closed without path or command leakage.
- The explicit DocsCheck profile runs once through the real authoritative route.
- The local-check result supplies the selected step evidence/check fact.
- The two-step workflow completes through ordinary executor semantics.

Existing project-validation, local-check, immutable-bundle, governance,
approval, report, provider, and runtime tests remain part of workspace validation.

## 8. Governed Phase Evidence

The initial governed run (`run-1786605616475081000-2`) was closed without source
changes after review found that adding a second serde-visible profile value was
outside its stated schema non-goal. Its 39-event trail completed with one
proof-bound approval and no retries or escalations.

The corrected implementation run is:

- workflow: `dg/runtime-composition`;
- run: `run-1786606027190141000-2`;
- approval: `approval/run-1786606027190141000-2/composition-approved`;
- presentation: `presentation/d3c006bd03e7ca8c`;
- approval outcome: granted under delegated maintainer authority.

## 9. Validation

Final validation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`.

Focused profile and executor tests also passed during implementation.

GitHub CI initially rejected the end-to-end profile test because its explicit
runtime facts classified a read-only skill as `LocalReversible`. The runtime
correctly failed closed with
`governance.proportional.derivation.side_effect_mismatch`. A governed blocker
fix changed only the test facts to `None`, the model's correct posture when
read-only work proposes no `SideEffect`; production derivation behavior was not
weakened. The focused test and the complete validation matrix then passed.

Blocker-fix evidence:

- workflow: `dg/blocker`;
- run: `run-1786608054779658000-2`;
- approval: `approval/run-1786608054779658000-2/fix-approved`;
- presentation: `presentation/10d4e521b1c53c20`;
- outcome: granted under delegated maintainer authority;
- event summary: 39 events, one proof-bound approval, no retries or escalations.

## 10. Out-Of-Kernel Work

The kernel governed phase scope, presentation proof, approval, and event trail.
Codex inspected code and docs, edited repository files, ran commands and tests,
and will perform git and pull-request operations outside the kernel. The kernel
did not execute the repository edits, validation commands, or GitHub actions.

## 11. Remaining Limitations

- `DocsCheck` is specific to the Workflow OS repository contract.
- Project activation remains limited to project validation.
- The explicit-facts route still requires trusted runtime facts for facts not
  produced by the selected local check.
- No CLI path selects this profile.
- No command family discovery or user-authored command execution exists.

## 12. Recommended Next Phase

Perform the authoritative DocsCheck profile runtime-composition review. Confirm
the additive serialized vocabulary and public explicit-facts route remain
appropriately closed before selecting another engineering-check or provider
mutation vertical slice.
