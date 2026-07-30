# Authoritative Runtime Flag Retirement Report

## 1. Executive Summary

The caller-classified authoritative runtime compatibility path is retired.
`run` now enters authoritative execution only from the validated project
declaration. `approve` now enters authoritative reassessment only from the
matching immutable run activation.

The scaffold option remains available because it authors the declaration
rather than granting runtime authority.

## 2. Scope Completed

- Removed the runtime flag from `run` and `approve` command models, help, and
  approval next-action rendering.
- Added stable fail-closed parser rejection for attempted runtime flag use.
- Required a validated project activation for authoritative run construction.
- Removed CLI injection of a preclassified sufficient-authority fact.
- Preserved authoritative quiet, visible, approval, denial, report, artifact,
  and approval-resume behavior under project-declared activation.
- Updated focused tests and current product documentation.

## 3. Scope Explicitly Not Completed

No automatic approval, inferred authority, actor RBAC, enterprise identity,
capability grants, OpenShell, sandbox execution, access material, providers,
SideEffect execution, external writes, schemas, new local-check profiles,
hosted behavior, or release changes were added.

## 4. Runtime Boundary

Fresh authoritative execution requires the current validated project
declaration. Approval resume requires its exact durable immutable activation.
Core remains responsible for deriving sufficient current authority inside the
closed same-call consumer.

The CLI no longer manufactures a sufficient-authority runtime fact.

## 5. Failure And Privacy

Retired flag use fails with
`cli.authoritative_governance.runtime_flag_retired` before run creation or
approval mutation. Error text contains migration guidance but omits governed
IDs and caller payloads.

## 6. Compatibility

This is a breaking change to an explicitly experimental runtime option.
Ordinary undeclared execution is unchanged. The
`init-repo-governance --authoritative-governance` scaffold option remains and
writes the supported project declaration.

## 7. Tests

Focused CLI coverage verifies:

- all authoritative route families through project-declared activation;
- immutable approval-resume activation;
- missing check-profile failure;
- runtime flag rejection for both `run` and `approve`;
- zero state mutation on rejection;
- bounded approval handoff commands; and
- unchanged scaffold opt-in behavior.

## 8. Validation

Validation commands and final results:

- `cargo fmt --all --check`: passed;
- `cargo check -p workflow-core --lib`: passed;
- `cargo check -p workflow-cli --bin workflow-os`: passed;
- direct binary probes for retired `run` and `approve` flags: passed with
  `cli.authoritative_governance.runtime_flag_retired`;
- focused CLI test target: compiled, then encountered the repository's known
  macOS post-launch integration-test stall before emitting test results;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: compiled and passed the first four CLI unit tests,
  then encountered the same macOS integration-test launch stall at
  `ci_read_only_example`; no local full-suite pass is claimed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- GitHub CI: required before merge.

## 9. Governed Phase Record

- workflow ID: `dg/runtime-composition`;
- run ID: `run-1785414984547843000-2`;
- approval ID:
  `approval/run-1785414984547843000-2/composition-approved`;
- presentation ID: `presentation/a14f7bdc3aea3a58`;
- approval outcome: granted under delegated-maintainer authority; and
- approval-presentation enforcement: proof persisted before execution.

Phase close completed with 39 events, one approval, zero retries, and zero
escalations. Repository edits, shell commands, validation, and later git/PR
operations remain out-of-kernel executor work and are disclosed here rather
than represented as kernel-executed activity.

## 10. Remaining Limitations

The project declaration represents workload-level activation for one closed
local validation profile. It is not actor-specific authorization, a general
capability grant, sandbox containment, or enterprise policy administration.

## 11. Recommended Next Phase

Resume scoped runtime authority and capability projection. OpenShell may later
be evaluated as an optional execution provider, but it must not become an
authority source and must not precede the capability boundary.
