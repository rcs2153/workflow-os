# Authoritative Governance Scaffold Opt-In Report

## 1. Executive Summary

The existing-repository scaffold now has one explicit opt-in for the accepted
authoritative proportional-governance path:

```sh
workflow-os init-repo-governance --authoritative-governance
```

The option writes the closed `observe_and_report` and
`workflow_os_project_validation` declaration into `workflow-os.yml`. Default
scaffolding remains undeclared and retains its prior output. Scaffolding still
executes no workflow or check and creates no runtime state.

## 2. Scope Completed

- Added the `--authoritative-governance` scaffold option.
- Generated the existing supported authoritative declaration only when
  selected.
- Added explicit enabled-profile output for normal and dry-run invocation.
- Made the command reject unknown or misspelled options before writing files.
- Preserved default manifest and output behavior when the option is absent.
- Verified the opted-in project validates.
- Verified `first-run --verbose` reports the declaration as supported and
  enforced.
- Updated roadmap, CLI, onboarding, product-contract, and agent-harness
  documentation.

## 3. Scope Explicitly Not Completed

This phase did not add:

- silent or inferred activation;
- new proportional-governance routes or policy semantics;
- arbitrary command discovery or execution;
- additional local-check profiles;
- provider execution or OpenShell integration;
- network access, credentials, SideEffect execution, or external writes;
- automatic approval;
- hosted or enterprise controls;
- workflow generation or promotion; or
- release posture changes.

## 4. CLI And Manifest Behavior

The supported option writes:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

Selected scaffold and dry-run output disclose:

```text
authoritative_execution: enabled
authoritative_execution_profile: observe_and_report
authoritative_execution_local_check_profile: workflow_os_project_validation
```

Without the option, no declaration or new output line is added.

## 5. Runtime Boundary

The scaffold command remains file generation only. Once generated, the
declaration activates the previously accepted authoritative path for later
`run` and matching `approve` calls:

- the fixed project-validation check is resolved;
- Core selects quiet, visible, approval-required, or denied posture;
- immutable run and approval-resume integrity remain active; and
- eligible terminal runs pass through the accepted WorkReport artifact gates.

The option grants no repository-command, provider, network, or external-write
authority.

## 6. Tests Added

Focused CLI tests prove:

- default scaffolding omits the declaration and preserves prior output;
- explicit opt-in writes the exact closed declaration;
- the opted-in project validates;
- first-run reports the declaration as enforced;
- opted-in dry-run writes no project files or state;
- an unknown or misspelled option fails before any scaffold write; and
- existing scaffold dry-run and default behavior remain intact.

Existing workspace, TypeScript SDK, contract, dogfood helper, integration, and
documentation checks remain green.

Full workspace validation initially exposed a pre-existing concurrent
approval-projection reconciliation defect in the accepted authoritative
artifact path. That blocker was governed and fixed without broadening the
scaffold feature. See
[Authoritative Artifact Concurrent Reconciliation Blocker Fix Report](AUTHORITATIVE_ARTIFACT_CONCURRENT_RECONCILIATION_BLOCKER_FIX_REPORT.md).

## 7. Validation Performed

Passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo test -p workflow-cli --test cli init_repo_governance`
- `npm run check`
- `npm run check:integrations` under the supported Node 20 toolchain
- `npm run check:docs`
- `git diff --check`

## 8. Governed Phase Record

- workflow id: `dg/implement`
- run id: `run-1785211968000576000-2`
- approval id:
  `approval/run-1785211968000576000-2/implementation-approved`
- presentation id: `presentation/1358129d9398286c`
- approval outcome: granted by delegated maintainer
- approval proof: persisted
- out-of-kernel work: repository edits, tests, and documentation were executed
  by Codex under the kernel-governed scope

## 9. Remaining Limitations

- The option is not the default.
- Only one closed authoritative profile/check combination is accepted.
- The generated first-run workflow remains a mockable approval/audit
  demonstration until a real skill handler is supplied.
- Workflow OS does not infer safe repository commands.
- OpenShell and other execution providers remain unimplemented.
- The first external onboarding evaluation of this option has not occurred.

## 10. Recommended Next Phase

Perform a focused maintainer review of the scaffold opt-in. If accepted, run an
external-repository onboarding evaluation that compares default and opted-in
scaffolds and verifies the user understands quiet success, escalation, fixed
validation authority, and terminal artifact posture before considering any
broader default.
