# Generic Explicit Local-Check Profile Source Plan Review

## 1. Executive Verdict

**Plan accepted with the profile-to-authoritative bridge correction; proceed
to Core implementation.**

The plan now defines an implementation-ready final prerequisite for the
explicit authoritative quiet-success CLI preview. It introduces one generic
Workflow OS project-validation profile without creating a generic shell
executor, and it accounts for the existing runtime's direct dependency on
`DocsCheckLocalHandler`.

## 2. Scope Verification

The plan stays within a Core-only, explicit, non-default boundary.

It does not authorize:

- CLI behavior;
- default registration;
- arbitrary command or shell input;
- command inference from repository metadata;
- automatic checks;
- schemas, SDKs, scaffolds, or examples;
- persistence or report artifacts;
- providers, OpenShell, containers, or credentials;
- SideEffect execution or writes;
- hosted behavior; or
- release changes.

## 3. Product Assessment

`workflow-os validate` is the right first generic profile.

It is meaningful for every Workflow OS project, does not depend on a language
ecosystem, does not require network access, and does not overclaim that project
tests or builds ran.

The profile will not by itself provide source-project quality evidence. That
limitation is acceptable because the first quiet-success preview needs one
honest generic check, not inferred automation.

## 4. Profile Source Assessment

The proposed resolved profile correctly binds:

- a closed profile ID;
- a canonical command contract;
- a stable skill ID and version;
- an explicit handler;
- immutable declaration inventory; and
- bounded safe metadata.

This is stronger than a handler-only registry helper. It prevents declaration,
registration, and execution from drifting into different command identities.

## 5. Command Contract Assessment

The proposed `workflow-os validate` template is fixed and allowlisted.

Using the selected project root as working context rather than arbitrary shell
text is appropriate. The contract preserves:

- sanitized minimal environment;
- disabled network;
- source-read-only posture;
- bounded redacted output; and
- no raw output persistence.

The serialized command contract should remain model-only. Explicit handler
construction and authoritative composition remain the execution boundary.

## 6. Authority Assessment

The plan correctly keeps `LocalSkillRegistry::new()` empty and rejects ambient
default registration.

Safe metadata discovery may recommend a profile but cannot select or authorize
one. An explicit profile selection is still only an execution opportunity:
immutable declaration binding, proportional governance, policy, authority, and
approval remain authoritative.

The preferred isolated-registry construction avoids the current general
registry's replacement semantics and should be used unless implementation
finds a smaller collision-rejecting boundary.

## 7. Runtime Compatibility Finding

The initial plan had one blocker: accepted authoritative runtime functions are
typed directly to `DocsCheckLocalHandler`.

A new `WorkflowOsProjectValidationLocalHandler` could not enter that path
without another runtime phase. Calling the profile source the final CLI
prerequisite would therefore have been inaccurate.

The plan was corrected to require one closed
profile-to-authoritative-composition bridge in the same implementation phase.
The bridge must preserve existing public `DocsCheck` APIs and must not become
an open arbitrary-handler authority surface.

With that correction, no planning blocker remains.

## 8. Immutable Bundle Assessment

The profile must provide the exact contract inventory used to publish the
immutable declaration set. Selected-step preflight must compare that stored
declaration against the same resolved contract and handler identity before any
process begins.

This preserves the repository's core rule that execution cannot outrun the
frozen run definition.

## 9. Proportional Governance Assessment

The profile supplies fresh authoritative check facts; it does not select the
route.

The existing two-axis model remains appropriate:

- execution disposition decides proceed, approval, or denial;
- disclosure disposition decides quiet or visible presentation.

Visible disclosure is therefore not a second blocking governance level. It is
an independently enforceable presentation obligation. The plan correctly
preserves that distinction.

## 10. Privacy And Failure Assessment

The planned boundary rejects:

- invalid executable or project roots;
- absent manifests;
- unsupported profile values;
- contract/skill/declaration mismatch;
- duplicate inventory;
- registration collision; and
- secret-like or unsafe values.

These failures occur before process execution and return stable non-leaking
errors. Debug output must redact paths, and no manifest contents, source
contents, environment values, or raw process output are copied.

## 11. Test Assessment

The future test plan is sufficient after the bridge correction. It covers:

- explicit resolution and empty defaults;
- canonical identity;
- no shell/path authority;
- preflight before process;
- deterministic registration and declaration inventory;
- handler success/failure/timeout;
- privacy and no persistence;
- closed authoritative bridge routing;
- quiet, visible, approval, denial, and report compatibility;
- one-check behavior; and
- full regression coverage.

## 12. Documentation And User Feedback

The plan responds appropriately to external evaluation:

- low-risk work should become less interruptive;
- evidence and audit posture must remain;
- first-run metadata must not become execution authority; and
- project-specific checks should be concrete only after a reviewed explicit
  profile exists.

The review also confirms that current proportional-governance configuration is
primarily derived from immutable workload facts plus explicit minima. The
future CLI profile selection should configure the check source, not allow a
caller to pick its preferred governance result.

## 13. Blockers

None after the runtime bridge correction.

## 14. Non-Blocking Follow-Ups

- Decide whether the current executable path is supplied by the CLI boundary
  or by a CLI-owned current-executable resolver.
- Decide whether profile IDs need explicit versions before adding a second
  profile.
- Keep ecosystem-specific profile recommendation separate from selection and
  authority.
- Revisit broader Node-version integration support separately; it is not a
  blocker for this Core phase.

## 15. Recommended Next Phase

Implement the generic explicit local-check profile source and its closed
profile-to-authoritative-composition bridge in Core.

Do not add the CLI flag in that phase. After implementation and focused review,
proceed directly to the explicit authoritative quiet-success CLI preview.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1785100009314370000-2`
- approval:
  `approval/run-1785100009314370000-2/review-scope-approved`
- presentation: `presentation/2d842ce7457a7b8a`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation: `npm run check:docs` and `git diff --check` passed
- approval-presentation enforcement: proof enforced
- out-of-kernel work: source inspection, plan correction, review authoring,
  documentation validation, and diff validation
- missing coverage: the kernel coordinated governance but did not inspect
  code, edit docs, run validation, or perform git/PR actions
