# Generic Explicit Local-Check Profile Canonical Contract Blocker Fix Report

## 1. Executive Summary

The generic explicit Workflow OS project-validation profile now fails closed
unless its public handler constructor receives the complete canonical
`workflow-os validate` contract.

The fix closes the one blocker found during focused implementation review. It
does not add CLI behavior, registration defaults, command inference,
persistence, providers, OpenShell, SideEffect execution, or writes.

## 2. Blocker Fixed

The public project-validation handler constructors accepted any model-valid
contract with the project-validation command kind and broad safe posture.
Command template validation already prevented arbitrary executables and
arguments, but other accepted fields such as timeout, output bounds, citation
kinds, or command ID could differ from the profile's built-in contract.

That allowed profile identity and evidence posture to drift from the claimed
complete canonical definition.

## 3. Implementation Approach

`WorkflowOsProjectValidationLocalHandler::new_with_process_runner(...)` now:

1. validates the supplied contract through the existing model boundary;
2. constructs the built-in canonical project-validation contract;
3. compares the complete validated contracts for equality; and
4. fails before path validation, handler storage, request construction, or
   process execution when any field differs.

The stable failure code is:

```text
local_check.profile.handler.contract_non_canonical
```

The error does not include caller-supplied contract values.

## 4. Authority Boundary

The fixed profile now binds one complete contract:

- command ID `local-check/workflow-os-validate`;
- kind `workflow_os_project_validation`;
- executable `workflow-os`;
- arguments `[validate]`;
- repository-root working directory;
- sanitized environment;
- disabled network;
- fixed timeout and output bounds;
- no source writes;
- bounded redaction posture; and
- fixed report citation kinds.

The selection resolver, resolved profile, registry identity, immutable
declaration inventory, authoritative preflight, handler, and report citation
therefore share the same canonical contract.

## 5. Privacy And Failure Posture

The fix stores no new caller data and adds no serialization surface.

Contract mismatch errors expose neither changed values nor executable,
project, environment, output, source, spec, provider, credential, token, or
path data. Rejection occurs before a process request exists.

## 6. Test Coverage

The blocker regression constructs a contract that:

- retains the canonical executable and argument template;
- remains valid under the general local-check model; but
- changes the canonical timeout.

Public handler construction rejects it with the stable mismatch code. The
test also proves the injected runner received no request.

Existing profile, DocsCheck, immutable-run, approval, report, adapter, CLI,
and runtime coverage remains part of the full workspace validation.

## 7. Validation Commands

The phase requires:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

All required commands passed.

## 8. Scope Explicitly Not Completed

The fix did not add:

- CLI or UI behavior;
- default registration or profile inference;
- arbitrary command or shell execution;
- workflow schema, SDK, scaffold, or example changes;
- local-check result persistence or report artifacts;
- providers, OpenShell, containers, credentials, or network access;
- SideEffect execution or writes;
- hosted behavior, reasoning lineage, or release changes.

## 9. Remaining Limitations

- The explicit profile still supports only Workflow OS project validation.
- The caller still supplies the executable and project root explicitly.
- The selected workflow must declare the exact canonical contract.
- Operator-facing quiet-success CLI behavior remains unimplemented.
- Project-specific checks are not inferred.

## 10. Recommended Next Phase

Perform a focused review of this blocker fix.

After acceptance, proceed to the explicit authoritative quiet-success CLI
preview without broadening profile authority or inferring commands from
repository metadata.

## 11. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1785104239844427000-2`
- approval:
  `approval/run-1785104239844427000-2/fix-approved`
- presentation: `presentation/7b09a409c9609c98`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation enforcement: proof enforced
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- out-of-kernel work: source inspection, focused implementation, tests,
  documentation, and validation
- missing coverage: the kernel coordinated governance but did not inspect
  code, edit files, run tests, or perform git and PR actions
