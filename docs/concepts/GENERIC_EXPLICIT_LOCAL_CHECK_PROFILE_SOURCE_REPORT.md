# Generic Explicit Local-Check Profile Source Report

## 1. Executive Summary

Workflow OS now has one generic, explicit, non-default local-check profile for
validating a selected Workflow OS project. The profile binds one fixed
`workflow-os validate` command contract, one stable handler identity, and the
exact immutable declaration inventory consumed by the authoritative
proportional-governance runtime.

The phase also replaces the accepted authoritative runtime's internal
`DocsCheckLocalHandler` dependency with a private closed handler boundary. The
existing public DocsCheck APIs remain unchanged, while the resolved
project-validation profile can enter the same quiet, visible, approval,
denial, and report paths.

No CLI behavior, default registration, inferred command execution,
persistence, artifacts, providers, OpenShell, SideEffect execution, or writes
were added.

## 2. Scope Completed

The phase added:

- `WorkflowOsProjectValidation` as a closed local-check command kind;
- a fixed source-read-only, network-disabled command contract;
- `ExplicitLocalCheckProfileId`;
- `ExplicitLocalCheckProfileSelection`;
- `ResolvedExplicitLocalCheckProfile`;
- `WorkflowOsProjectValidationLocalHandler`;
- collision-rejecting explicit profile registration;
- exact immutable declaration inventory exposure;
- a private Core-owned authoritative handler boundary;
- an additive authoritative route helper for resolved profiles;
- an additive report-bearing helper for resolved profiles; and
- focused identity, collision, privacy, routing, report, and one-check tests.

## 3. Scope Explicitly Not Completed

The phase did not add:

- CLI flags or quiet-success operator output;
- automatic or default profile selection;
- shell strings or arbitrary command execution;
- repository metadata inference;
- project tests, builds, lint, or ecosystem-specific checks;
- workflow schema, SDK, scaffold, or example changes;
- local-check result persistence;
- report artifacts or automatic report generation;
- providers, adapters, OpenShell, containers, or credentials;
- network access;
- source writes or SideEffect execution;
- hosted or distributed behavior; or
- release posture changes.

## 4. Model And API Summary

`ExplicitLocalCheckProfileSelection::workflow_os_project_validation()` creates
the only supported selection. Resolution requires an explicitly supplied
Workflow OS executable and project root.

The resolved profile exposes bounded access to:

- its closed profile ID;
- the canonical command contract;
- stable skill ID `local/workflow-os-validate`;
- stable skill version `v0`; and
- the exact local-check declaration inventory.

`LocalSkillRegistry::register_resolved_explicit_local_check_profile(...)`
installs the resolved handler only when explicitly called and rejects
collisions.

The additive authoritative helpers are:

```text
route_authoritative_explicit_local_check_profile_governance(...)
execute_with_authoritative_explicit_local_check_profile_governance_report(...)
```

Existing public DocsCheck helpers retain their signatures and behavior.

## 5. Command And Handler Boundary

The canonical contract is:

```text
command_id: local-check/workflow-os-validate
command_kind: workflow_os_project_validation
executable: workflow-os
arguments: [validate]
working_directory: repository_root
environment: sanitized
network: disabled
side_effects: source_read_only
```

The serialized contract remains model-only. Execution authority exists only
when a caller explicitly resolves the profile and uses the closed
authoritative runtime path.

Resolution verifies the executable and requires `workflow-os.yml` at the
selected project root. Errors are stable and do not expose paths.

## 6. Immutable Identity And Authority

The resolved profile is the common source for registration, command identity,
skill identity, and immutable declaration inventory.

Authoritative preflight compares the workflow-selected declaration against the
same canonical contract and binds the actual local-check handler skill
identity into the immutable run bundle. A declaration, command, skill, or
profile mismatch fails before process execution or run events.

Safe metadata or workflow discovery cannot select this profile or authorize
execution. The ordinary registry remains empty by default.

## 7. Runtime And Report Behavior

The private authoritative handler boundary is closed to Core-owned handlers.
It does not expose a public arbitrary handler or command authority surface.

The resolved project-validation profile uses the existing authoritative
pipeline:

1. preflight immutable declarations and workload facts;
2. execute the fixed check once;
3. derive a source-bound governance assessment;
4. select quiet, visible, approval, or denial behavior; and
5. generate a terminal in-memory report when the selected route is terminal.

The report helper constructs its local-check citation from the same result
that informed governance. It does not execute a second report-only check.

## 8. Privacy And Failure Posture

The profile does not accept raw command text, arbitrary arguments, environment
values, source contents, spec contents, parser payloads, provider payloads,
credentials, authorization headers, private keys, or tokens.

Process output remains bounded and redacted through the existing local-check
handler boundary. Raw output is not persisted.

`Debug` implementations expose posture rather than executable or project
paths. Invalid roots, unsupported command kinds, collisions, and declaration
mismatches return stable non-leaking errors.

## 9. Test Coverage

Focused tests cover:

- fixed command identity and source-read-only posture;
- all command-kind templates;
- explicit resolution without process execution;
- collision-rejecting registration;
- exact declaration inventory;
- invalid project root failure without path leakage;
- closed authoritative quiet routing;
- source-bound governance assessment;
- report generation from the same-call result;
- exact result kind and stable report citation;
- no second check execution;
- durable run-event equality; and
- existing DocsCheck behavior.

The focused local-check and executor suites passed with 361 tests passing and
two intentional opt-in live tests ignored.

## 10. Validation Commands

Required phase-close validation:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

All required commands passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

The first full workspace run exposed a preflight regression from the handler
identity refactor: an unknown selected workflow step could reach process
execution. The implementation now resolves the selected step and its canonical
skill from the immutable bundle before binding the actual local-check handler
identity. The failing regression and the full workspace suite both pass after
that correction.

## 11. External Evaluation Alignment

Fresh-pull user evaluation confirms that Workflow OS's honest local kernel,
first-run posture, approval boundaries, and durable event trail are credible.
The dominant product pressure is now reducing ceremony for low-risk work while
preserving evidence.

This phase responds directly by making one concrete low-risk validation source
available to the accepted authoritative proportional-governance pipeline. It
does not infer project commands or weaken execution authority.

Node 24 integration-check behavior and the duplicate missing-manifest
diagnostic remain separate non-blocking onboarding issues. They do not justify
broadening this Core runtime phase.

## 12. Remaining Limitations

- Only Workflow OS project validation is supported.
- The executable and project root are supplied explicitly.
- The selected workflow must declare the exact canonical check contract.
- The CLI does not expose the profile.
- Project-specific checks are not inferred or run.
- Report output remains local and in memory.
- OpenShell is not integrated.

## 13. Recommended Next Phase

Perform a focused maintainer review of this implementation.

After acceptance, proceed to the explicit authoritative quiet-success CLI
preview. Keep the CLI path opt-in, preserve the explicit profile boundary, and
do not turn repository metadata into execution authority.

## 14. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1785100203603888000-2`
- approval:
  `approval/run-1785100203603888000-2/composition-approved`
- presentation: `presentation/f68cbf9e1ce69ad5`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation enforcement: proof enforced
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation: focused suites, formatting, warning-denied Clippy, full workspace
  tests, docs checks, and diff checks passed
- out-of-kernel work: source inspection, Rust implementation, tests,
  documentation, and validation
- missing coverage: the kernel coordinated governance but did not edit files,
  run tests, or perform git and PR actions
