# Authoritative Governance Scaffold Runtime Contract Blocker Fix Report

## 1. Executive Summary

The external-repository evaluation found that the explicit authoritative
scaffold selected the closed project-validation profile but did not declare the
matching workflow-step check requirement. The generated project validated and
reported enforced posture, then failed closed before run creation.

The scaffold now emits the existing canonical
`local-check/workflow-os-validate` requirement only when
`--authoritative-governance` is selected. Default scaffold output and runtime
behavior remain unchanged.

## 2. Blocker Fixed

Before the fix:

```text
workflow-os init-repo-governance --authoritative-governance
workflow-os --mock-all-local-skills run local/first-run-governance
```

failed with:

```text
cli.authoritative_governance.check_profile_missing
```

The project-level profile selection and generated workflow contract were
incomplete as a pair.

## 3. Implementation

`repo_governance_workflow(...)` now accepts the explicit scaffold posture.
When authoritative governance is disabled, it returns the existing workflow
unchanged. When enabled, it adds exactly one required project-validation
declaration with:

- the fixed command ID;
- kernel-observed local-process assurance;
- passed-only acceptance;
- no reuse;
- exact immutable-run binding;
- no truncation;
- disabled network; and
- no source writes.

The implementation does not infer repository commands or add another check
profile.

## 4. Runtime Proof

A clean disposable TypeScript-style repository was scaffolded after rebuilding
the CLI. The generated project:

- preserved existing `AGENTS.md` guidance;
- validated;
- executed the fixed project validation;
- selected the blocking proportional-governance route;
- presented and persisted the governance approval;
- preserved the separate workflow-step approval;
- completed one mock skill invocation; and
- persisted exactly one terminal WorkReport artifact.

The final inspect output contained 20 ordered events, two approvals, one local
check result reference, and one WorkReport artifact.

## 5. Compatibility And Privacy

- Default generated workflow content has no local-check declaration.
- Dry-run remains non-writing.
- Unknown options still fail before writes.
- The check contract is fixed and payload-free.
- No source contents, command output, provider payloads, paths, credentials, or
  tokens are added to reports or errors.

## 6. Tests

Focused CLI coverage proves:

- default scaffolding omits the authoritative declaration and check;
- explicit scaffolding includes the complete canonical check;
- the explicit project validates and reports enforced posture;
- the generated authoritative workflow reaches its governance approval;
- the governance approval does not collapse the workflow approval;
- the second approval completes the run; and
- exactly one WorkReport artifact is persisted.

## 7. Governed Phase Record

- evaluation workflow: `dg/review`
- evaluation run: `run-1785218225110625000-2`
- evaluation approval:
  `approval/run-1785218225110625000-2/review-scope-approved`
- blocker workflow: `dg/blocker`
- blocker run: `run-1785218523819759000-2`
- blocker approval: `approval/run-1785218523819759000-2/fix-approved`
- blocker presentation: `presentation/8e56d8a31327fc56`
- approval outcome: granted by delegated maintainer
- approval proof: persisted
- out-of-kernel work: code edits, disposable repository execution, tests, and
  documentation were performed by Codex under the kernel-governed scope

## 8. Validation

The following checks passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --quiet`
- focused authoritative scaffold CLI regression tests
- focused scaffold compatibility tests
- `npm run check`
- `npm run check:integrations` under the repository-pinned Node 20 toolchain
- `git diff --check`
- clean disposable-repository scaffold, validation, two-approval execution,
  inspect, and persisted-artifact proof

An earlier redirected workspace-test invocation was interrupted while
diagnosing unusually slow integration-test process startup and is not counted
as validation evidence. The subsequent standard workspace command completed
successfully.

## 9. Remaining Limitations

- Authoritative scaffolding remains explicit opt-in.
- The generated workflow uses a mockable skill for approval/audit
  demonstration.
- Two approvals are expected because proportional governance and the workflow
  both independently require review.
- No arbitrary repository command, provider, OpenShell sandbox, SideEffect
  execution, or external write is enabled.

## 10. Recommended Next Phase

Perform a focused maintainer review of this blocker fix. Do not broaden default
activation or execution providers before that review accepts the complete
scaffold-to-runtime boundary.
