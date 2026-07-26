# Generic Explicit Local-Check Profile Canonical Contract Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to explicit authoritative quiet-success CLI
preview.**

The public Workflow OS project-validation handler now accepts only the
complete canonical contract. Contract drift fails before handler storage,
request construction, or process execution.

## 2. Scope Verification

The fix stayed within the approved blocker boundary.

It did not add:

- CLI or UI behavior;
- default registration or inferred commands;
- runtime approval semantic changes;
- workflow schema, SDK, scaffold, or example changes;
- persistence or report artifacts;
- providers, OpenShell, containers, or credentials;
- SideEffect execution or writes;
- hosted behavior, reasoning lineage, or release changes.

## 3. Original Blocker Restatement

The public project-validation handler constructors accepted a caller-supplied
model-valid contract. General validation already fixed the executable and
arguments for the command kind, and the constructor checked broad safe
posture, but fields such as command ID, timeout, output bounds, redaction
posture, and citation kinds could differ from the profile's complete built-in
definition.

That permitted canonical profile identity and evidence posture to drift before
the first operator-facing exposure.

## 4. Fix Approach Assessment

The fix is minimal and idiomatic.

After normal contract validation, the constructor creates
`workflow_os_project_validation_model_only()` and compares the complete
contracts for equality. Any mismatch returns one stable profile-specific
validation error.

This preserves the general local-check model while making the selected
profile's authority stricter than the broader model vocabulary.

## 5. Authority And Ordering Assessment

The rejection occurs before:

- executable or project-root prerequisite checks;
- handler construction or registry installation;
- immutable declaration publication;
- process request construction;
- process runner invocation; and
- workflow event or report creation.

The selection resolver continues to construct the canonical contract itself.
The resolved profile, stable handler identity, declaration inventory,
authoritative preflight, runtime observation, governance assessment, and
report citation therefore share one complete contract.

## 6. Error And Privacy Assessment

The stable mismatch code is:

```text
local_check.profile.handler.contract_non_canonical
```

The error does not include changed contract values. No new `Debug`,
serialization, persistence, path, environment, source-content, provider,
credential, token, or output surface was added.

## 7. Regression Assessment

Existing valid profile resolution, registration, authoritative routing, and
same-call report behavior remain unchanged.

Existing DocsCheck APIs and the ordinary empty registry posture remain
unchanged. The full workspace suite covers immutable-run, approval,
WorkReport, EvidenceReference, SideEffect, adapter, CLI, and runtime behavior.

## 8. Test Quality Assessment

The focused regression uses a contract that remains valid under the general
local-check model but changes the built-in timeout. Public handler
construction:

- rejects the contract with the stable mismatch code;
- does not echo the changed value; and
- does not call the injected process runner.

Complete equality makes this representative test sufficient for all canonical
fields. Existing command-template tests separately prove that arbitrary
executables and arguments fail general model validation.

No blocking test gap remains.

## 9. Documentation Assessment

The original implementation report and review are preserved. The review's
fix-forward section, blocker-fix report, implementation plan, and roadmap now
state that complete canonical equality is enforced and CLI exposure remains
deferred until this review.

Fresh-pull user feedback continues to support the next phase: reduce ceremony
for low-risk work while preserving real evidence and honest execution
boundaries. This fix strengthens that path rather than adding ceremony.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add explicit-profile visible, approval, denial, and concise human-output
  tests at the CLI boundary.
- Keep profile selection explicit and non-default.
- Do not infer project-specific execution commands from safe metadata.
- Diagnose Node 24 integration-check behavior separately from kernel
  correctness.
- Remove the duplicate missing-manifest diagnostic in a bounded CLI polish
  phase.

## 12. Recommended Next Phase

Implement the explicit authoritative quiet-success CLI preview.

The CLI path should be opt-in, resolve the closed project-validation profile,
surface quiet success concisely, preserve visible/approval/denial behavior,
and cite the same authoritative result. It must not infer commands, enable
default execution, persist reports, or broaden writes.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1785106504283389000-2`
- approval:
  `approval/run-1785106504283389000-2/review-scope-approved`
- presentation: `presentation/c81d73245b6980b8`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation enforcement: proof enforced
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation:
  - `cargo fmt --all --check` passed
  - `cargo clippy --workspace --all-targets -- -D warnings` passed
  - `cargo test --workspace` passed
  - `npm run check:docs` passed
  - `git diff --check` passed
- out-of-kernel work: source inspection, focused maintainer review,
  documentation, and validation
- missing coverage: the kernel coordinated governance but did not inspect
  code, edit files, run tests, or perform git and PR actions
